//! A work-alike of the `dav1d` command line tool.
//!
//! Decodes AV1 using the safe `dav1d` crate, reads the same container formats
//! and writes the same output formats as `dav1d`, with the options mirrored in
//! [`cli`].

mod cli;
mod demux;
mod mux;

use std::error::Error as StdError;
use std::fs::File;
use std::io::{self, BufWriter, IsTerminal, Write};
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

use dav1d::{dav1d_sys, Decoder};
use winnow_args::Args;

use crate::cli::{Cli, Config};
use crate::demux::{Demuxer, Kind as DemuxerKind, Packet};
use crate::mux::Output;

type Result<T> = std::result::Result<T, Box<dyn StdError>>;

fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", dav1d_version());
        return ExitCode::SUCCESS;
    }

    let config = match cli.resolve() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = run(config) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

struct Input {
    demuxer: Box<dyn Demuxer>,
    buffered: Option<Packet>,
}

impl Input {
    fn open(path: &Path, forced: Option<DemuxerKind>) -> io::Result<Self> {
        Ok(Self {
            demuxer: demux::open(path, forced)?,
            buffered: None,
        })
    }

    fn next(&mut self) -> io::Result<Option<Packet>> {
        match self.buffered.take() {
            Some(packet) => Ok(Some(packet)),
            None => self.demuxer.read_packet(),
        }
    }
}

fn run(config: Config) -> Result<()> {
    let mut input = Input::open(&config.input, config.demuxer)?;
    let fps = input.demuxer.fps();
    let total = input.demuxer.num_frames();

    // Skip whole packets, then any packet without a sequence header so that
    // decoding can start from a usable point.
    for _ in 0..config.skip {
        if input.next()?.is_none() {
            return Err("No data decoded".into());
        }
    }
    if config.skip > 0 {
        let mut skipped = 0;
        loop {
            let Some(packet) = input.next()? else {
                return Err("No data decoded".into());
            };
            if has_sequence_header(&packet.data) {
                input.buffered = Some(packet);
                break;
            }
            skipped += 1;
        }
        if skipped > 0 && !config.quiet {
            eprintln!("skipped {skipped} packets due to missing sequence header");
        }
    }

    if !config.quiet {
        eprintln!(
            "dav1d-tools {} (dav1d {})",
            env!("CARGO_PKG_VERSION"),
            dav1d_version()
        );
    }

    let mut frametimes = match config.frametimes.as_deref() {
        Some(path) => Some(BufWriter::new(File::create(path)?)),
        None => None,
    };

    let mut decoder = Decoder::with_settings(&config.settings)?;
    let mut output: Option<Output> = None;
    let mut decoded: u64 = 0;
    let mut pending = false;
    let mut eof = false;
    let start = Instant::now();
    let mut last_ns: u64 = 0;
    let istty = io::stderr().is_terminal();
    let limit = config.limit.filter(|limit| *limit != 0);

    loop {
        if pending {
            match decoder.send_pending_data() {
                Ok(()) => pending = false,
                Err(dav1d::Error::Again) => {}
                Err(error) => return Err(format!("Error decoding frame: {error}").into()),
            }
        } else if !eof {
            match input.next()? {
                Some(packet) => {
                    let Packet {
                        data,
                        offset,
                        timestamp,
                    } = packet;
                    match decoder.send_data(data, offset, timestamp, None) {
                        Ok(()) => {}
                        Err(dav1d::Error::Again) => pending = true,
                        Err(error) => return Err(format!("Error decoding frame: {error}").into()),
                    }
                }
                None => eof = true,
            }
        }

        match decoder.get_picture() {
            Ok(picture) => {
                if output.is_none() {
                    output = Some(Output::new(
                        config.muxer,
                        config.output.clone(),
                        config.per_frame,
                        fps,
                    )?);
                }
                output
                    .as_mut()
                    .expect("created above")
                    .write_frame(&picture)?;
                decoded += 1;

                let now_ns = start.elapsed().as_nanos() as u64;
                if let Some(frametimes) = frametimes.as_mut() {
                    writeln!(frametimes, "{}", now_ns - last_ns)?;
                    frametimes.flush()?;
                }
                last_ns = now_ns;

                if !config.quiet {
                    print_stats(istty, decoded, total, now_ns, fps);
                }

                if let Some(limit) = limit {
                    if decoded >= u64::from(limit) {
                        break;
                    }
                }
            }
            Err(dav1d::Error::Again) => {
                if eof && !pending {
                    break;
                }
            }
            Err(error) => return Err(format!("Error decoding frame: {error}").into()),
        }
    }

    drop(frametimes);

    let mut output = output.ok_or("No data decoded")?;
    let digest = output.finish(config.verify.is_none())?;

    if let Some(expected) = config.verify.as_deref() {
        let digest = digest.ok_or("No checksum computed")?;
        if !checksum_matches(expected, &digest) {
            return Err(
                format!("MD5 verification failed: expected {expected}, got {digest}").into(),
            );
        }
    }

    Ok(())
}

fn has_sequence_header(data: &[u8]) -> bool {
    let mut header = std::mem::MaybeUninit::<dav1d_sys::Dav1dSequenceHeader>::uninit();
    unsafe {
        dav1d_sys::dav1d_parse_sequence_header(header.as_mut_ptr(), data.as_ptr(), data.len()) == 0
    }
}

fn checksum_matches(expected: &str, actual: &str) -> bool {
    match expected.get(..32) {
        Some(expected) => actual.eq_ignore_ascii_case(expected),
        None => false,
    }
}

fn print_stats(istty: bool, decoded: u64, total: Option<u32>, elapsed_ns: u64, fps: (u32, u32)) {
    let mut line = String::new();
    if istty {
        line.push('\r');
    }

    match total {
        Some(total) if total != 0 => line.push_str(&format!(
            "Decoded {decoded}/{total} frames ({:.1}%)",
            100.0 * decoded as f64 / f64::from(total)
        )),
        _ => line.push_str(&format!("Decoded {decoded} frames")),
    }

    let decoded_fps = if elapsed_ns == 0 {
        0.0
    } else {
        1e9 * decoded as f64 / elapsed_ns as f64
    };
    if fps.0 != 0 && fps.1 != 0 {
        let input_fps = f64::from(fps.0) / f64::from(fps.1);
        line.push_str(&format!(
            " - {decoded_fps:.2}/{input_fps:.2} fps ({:.2}x)",
            decoded_fps / input_fps
        ));
    } else {
        line.push_str(&format!(" - {decoded_fps:.2} fps"));
    }

    if !istty {
        line.push('\n');
    }
    eprint!("{line}");
}

fn dav1d_version() -> String {
    unsafe {
        let version = dav1d_sys::dav1d_version();
        if version.is_null() {
            "unknown".to_owned()
        } else {
            std::ffi::CStr::from_ptr(version)
                .to_string_lossy()
                .into_owned()
        }
    }
}
