//! Command line interface, mirroring `dav1d`'s options.

use std::path::{Path, PathBuf};

use dav1d::{DecodeFrameType, InloopFilterType, Settings};
use winnow_args::Args;

use crate::demux::Kind as DemuxerKind;
use crate::mux::MuxerKind;

/// Decode AV1 streams with dav1d.
///
/// A work-alike of the `dav1d` command line tool, built on the safe `dav1d`
/// crate.
#[derive(Args, Debug)]
#[arg(
    name = "dav1d-tools",
    disable_version_flag,
    after_help = "Not implemented: --realtime, --realtimecache, --cpumask, --negstride and the 'xxh3' muxer."
)]
pub struct Cli {
    /// Print version and exit.
    #[arg(short = 'v', long)]
    pub version: bool,

    /// Input file.
    #[arg(short = 'i', long)]
    pub input: Option<PathBuf>,

    /// Output file. A `%n` in the name writes one file per frame, `%w` and
    /// `%h` are then replaced by the frame size.
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Force the demuxer type.
    #[arg(long, value_name = "NAME", choices("ivf", "annexb", "section5"))]
    pub demuxer: Option<String>,

    /// Force the muxer type, use `frame` as prefix to write per-frame files.
    #[arg(long, value_name = "NAME")]
    pub muxer: Option<String>,

    /// Disable status messages.
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Dump frame times to a file.
    #[arg(long)]
    pub frametimes: Option<PathBuf>,

    /// Stop decoding after `N` frames.
    #[arg(short = 'l', long, value_name = "N")]
    pub limit: Option<u32>,

    /// Skip decoding of the first `N` frames.
    #[arg(short = 's', long, value_name = "N", default = "0")]
    pub skip: u32,

    /// Number of threads (0 means automatic).
    #[arg(long, value_name = "N", default = "0")]
    pub threads: u32,

    /// Maximum frame delay, capped at the number of threads.
    #[arg(long, value_name = "N", default = "0")]
    pub framedelay: u32,

    /// Enable film grain application (default: 1, except for checksum muxers).
    #[arg(long, value_name = "0|1")]
    pub filmgrain: Option<u32>,

    /// Select an operating point of a scalable AV1 bitstream (0 - 31).
    #[arg(long, value_name = "N", default = "0")]
    pub oppoint: u32,

    /// Output all spatial layers of a scalable AV1 bitstream.
    #[arg(long, value_name = "0|1")]
    pub alllayers: Option<u32>,

    /// Stop decoding if the frame size exceeds the specified limit.
    #[arg(long, value_name = "N|WxH")]
    pub sizelimit: Option<String>,

    /// Abort decoding on standard compliance violations.
    #[arg(long, value_name = "0|1")]
    pub strict: Option<u32>,

    /// Verify the decoded md5, implies the `md5` muxer and no output.
    #[arg(long, value_name = "MD5")]
    pub verify: Option<String>,

    /// Whether to output invisible (alt-ref) frames.
    #[arg(long, value_name = "0|1")]
    pub outputinvisible: Option<u32>,

    /// Which in-loop filters to enable: none, (no)deblock, (no)cdef,
    /// (no)restoration or all.
    #[arg(long, value_name = "FILTERS")]
    pub inloopfilters: Option<String>,

    /// Which frame types to decode: reference, intra, key or all.
    #[arg(long, value_name = "TYPE")]
    pub decodeframetype: Option<String>,
}

/// Everything `main` needs after the command line has been validated.
pub struct Config {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub demuxer: Option<DemuxerKind>,
    pub muxer: MuxerKind,
    pub per_frame: bool,
    pub quiet: bool,
    pub frametimes: Option<PathBuf>,
    pub limit: Option<u32>,
    pub skip: u32,
    pub verify: Option<String>,
    pub settings: Settings,
}

impl Cli {
    /// Validates the command line and turns it into a [`Config`].
    pub fn resolve(&self) -> Result<Config, String> {
        let (prefixed_per_frame, muxer_name) = match self.muxer.as_deref() {
            Some(name) if name.starts_with("frame") => (true, name[5..].trim_start()),
            Some(name) => (false, name),
            None => (false, ""),
        };

        let mut verify = self.verify.clone();
        if verify.is_some() {
            if self.output.is_some() {
                return Err("Verification (--verify) requires no output file".into());
            }
            if !muxer_name.is_empty() && muxer_name != "md5" {
                return Err("Verification (--verify) requires the md5 muxer".into());
            }
            verify = Some(verify.unwrap());
        }

        let (muxer, per_frame) = if verify.is_some() {
            (MuxerKind::Md5, false)
        } else if !muxer_name.is_empty() {
            let kind = MuxerKind::from_name(muxer_name)
                .ok_or_else(|| format!("Failed to find muxer named \"{muxer_name}\""))?;
            (kind, prefixed_per_frame)
        } else if let Some(output) = self.output.as_deref() {
            if output == Path::new("/dev/null") {
                (MuxerKind::Null, false)
            } else if let Some(kind) = MuxerKind::from_path(output) {
                (kind, has_per_frame_pattern(output))
            } else {
                return Err(format!(
                    "Failed to find muxer for extension of {}",
                    output.display()
                ));
            }
        } else {
            (MuxerKind::Null, false)
        };

        if verify.is_none() && self.output.is_none() && muxer_name != "null" {
            return Err("Output file (-o/--output) is required".into());
        }

        let mut settings = Settings::new();
        settings.set_n_threads(self.threads);
        settings.set_max_frame_delay(self.framedelay);
        settings.set_operating_point(self.oppoint);
        settings.set_all_layers(self.alllayers.map(|v| v != 0).unwrap_or(true));
        // dav1d's command line tool defaults to strict compliance, unlike the
        // library.
        settings.set_strict_std_compliance(self.strict.map(|v| v != 0).unwrap_or(true));
        settings.set_output_invisible_frames(self.outputinvisible.map(|v| v != 0).unwrap_or(false));
        settings.set_apply_grain(
            self.filmgrain
                .map(|v| v != 0)
                .unwrap_or(muxer != MuxerKind::Md5),
        );

        if let Some(limit) = self.sizelimit.as_deref() {
            settings.set_frame_size_limit(parse_size_limit(limit)?);
        }
        if let Some(filters) = self.inloopfilters.as_deref() {
            settings.set_inloop_filters(parse_inloop_filters(filters)?);
        }
        if let Some(frame_type) = self.decodeframetype.as_deref() {
            settings.set_decode_frame_type(parse_decode_frame_type(frame_type)?);
        }

        Ok(Config {
            input: self
                .input
                .clone()
                .ok_or("Input file (-i/--input) is required")?,
            output: self.output.clone(),
            demuxer: self
                .demuxer
                .as_deref()
                .map(|name| DemuxerKind::from_name(name).expect("checked by the parser")),
            muxer,
            per_frame,
            quiet: self.quiet,
            frametimes: self.frametimes.clone(),
            limit: self.limit,
            skip: self.skip,
            verify,
            settings,
        })
    }
}

/// All the in-loop filters.
pub fn all_inloop_filters() -> InloopFilterType {
    InloopFilterType::DEBLOCK | InloopFilterType::CDEF | InloopFilterType::RESTORATION
}

fn parse_inloop_filters(value: &str) -> Result<InloopFilterType, String> {
    let all = all_inloop_filters();
    let filters = match value {
        "none" => InloopFilterType::empty(),
        "deblock" => InloopFilterType::DEBLOCK,
        "nodeblock" => all & !InloopFilterType::DEBLOCK,
        "cdef" => InloopFilterType::CDEF,
        "nocdef" => all & !InloopFilterType::CDEF,
        "restoration" => InloopFilterType::RESTORATION,
        "norestoration" => all & !InloopFilterType::RESTORATION,
        "all" => all,
        _ => {
            return Err(format!(
                "Invalid argument \"{value}\" for --inloopfilters; should be any of none, \
                 deblock, nodeblock, cdef, nocdef, restoration, norestoration or all"
            ))
        }
    };
    Ok(filters)
}

fn parse_decode_frame_type(value: &str) -> Result<DecodeFrameType, String> {
    match value {
        "all" => Ok(DecodeFrameType::All),
        "reference" => Ok(DecodeFrameType::Reference),
        "intra" => Ok(DecodeFrameType::Intra),
        "key" => Ok(DecodeFrameType::Key),
        _ => Err(format!(
            "Invalid argument \"{value}\" for --decodeframetype; should be any of all, \
             reference, intra or key"
        )),
    }
}

/// Parses either a pixel count (`N`) or a dimension (`WxH`).
fn parse_size_limit(value: &str) -> Result<u32, String> {
    let invalid = || {
        format!("Invalid argument \"{value}\" for --sizelimit; should be an integer or dimension")
    };

    let (w, h) = match value.split_once(['x', 'X']) {
        Some((w, h)) => (w, Some(h)),
        None => (value, None),
    };
    let w: u64 = w.parse().map_err(|_| invalid())?;
    let limit = match h {
        Some(h) => w.checked_mul(h.parse::<u64>().map_err(|_| invalid())?),
        None => Some(w),
    }
    .filter(|limit| *limit < u32::MAX as u64)
    .ok_or_else(invalid)?;
    Ok(limit as u32)
}

/// Whether `path` contains a `%n` frame-number pattern.
///
/// `dav1d` only switches to one file per frame automatically on `%n`;
/// `%w` and `%h` are expanded once that mode is on.
fn has_per_frame_pattern(path: &Path) -> bool {
    let Some(path) = path.to_str() else {
        return false;
    };
    let mut rest = path;
    while let Some(pos) = rest.find('%') {
        rest = &rest[pos + 1..];
        let rest_trimmed = rest.trim_start_matches(|c: char| c.is_ascii_digit());
        if rest_trimmed.starts_with('n') {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(words: &[&str]) -> Cli {
        Cli::parse_from(words.iter().copied()).expect("command line should parse")
    }

    #[test]
    fn version_does_not_require_an_input() {
        let cli = parse(&["-v"]);
        assert!(cli.version);
        assert!(cli.input.is_none());
    }

    #[test]
    fn input_is_required() {
        assert!(parse(&["-o", "out.y4m"]).resolve().is_err());
    }

    #[test]
    fn defaults_to_y4m_from_the_extension() {
        let config = parse(&["-i", "in.ivf", "-o", "out.y4m"]).resolve().unwrap();
        assert_eq!(config.muxer, MuxerKind::Y4m);
        assert!(!config.per_frame);
        assert!(config.settings.get_apply_grain());
        assert!(config.settings.get_strict_std_compliance());
        assert!(config.settings.get_all_layers());
    }

    #[test]
    fn md5_muxer_disables_film_grain_unless_asked() {
        let config = parse(&["-i", "in.ivf", "-o", "out.md5"]).resolve().unwrap();
        assert_eq!(config.muxer, MuxerKind::Md5);
        assert!(!config.settings.get_apply_grain());

        let config = parse(&["-i", "in.ivf", "-o", "out.md5", "--filmgrain", "1"])
            .resolve()
            .unwrap();
        assert!(config.settings.get_apply_grain());
    }

    #[test]
    fn verify_implies_md5_and_needs_no_output() {
        let config = parse(&[
            "-i",
            "in.ivf",
            "--verify",
            "0123456789abcdef0123456789abcdef",
        ])
        .resolve()
        .unwrap();
        assert_eq!(config.muxer, MuxerKind::Md5);
        assert!(config.verify.is_some());
        assert!(config.output.is_none());
    }

    #[test]
    fn verify_rejects_an_output_file() {
        let cli = parse(&[
            "-i",
            "in.ivf",
            "-o",
            "out.y4m",
            "--verify",
            "0123456789abcdef0123456789abcdef",
        ]);
        assert!(cli.resolve().is_err());
    }

    #[test]
    fn output_is_required_unless_null() {
        assert!(parse(&["-i", "in.ivf"]).resolve().is_err());
        let config = parse(&["-i", "in.ivf", "--muxer", "null"])
            .resolve()
            .unwrap();
        assert_eq!(config.muxer, MuxerKind::Null);
    }

    #[test]
    fn frame_prefix_and_patterns_write_per_frame_files() {
        let config = parse(&[
            "-i",
            "in.ivf",
            "-o",
            "out.y4m",
            "--muxer",
            "frame yuv4mpeg2",
        ])
        .resolve()
        .unwrap();
        assert!(config.per_frame);

        let config = parse(&["-i", "in.ivf", "-o", "frame%03n.y4m"])
            .resolve()
            .unwrap();
        assert!(config.per_frame);
        assert_eq!(config.muxer, MuxerKind::Y4m);
    }

    #[test]
    fn parses_settings() {
        let config = parse(&[
            "-i",
            "in.ivf",
            "-o",
            "out.y4m",
            "--threads",
            "4",
            "--framedelay",
            "2",
            "--oppoint",
            "3",
            "--alllayers",
            "0",
            "--sizelimit",
            "1920x1080",
            "--strict",
            "0",
            "--outputinvisible",
            "1",
            "--inloopfilters",
            "nodeblock",
            "--decodeframetype",
            "key",
        ])
        .resolve()
        .unwrap();
        assert_eq!(config.settings.get_n_threads(), 4);
        assert_eq!(config.settings.get_max_frame_delay(), 2);
        assert_eq!(config.settings.get_operating_point(), 3);
        assert!(!config.settings.get_all_layers());
        assert_eq!(config.settings.get_frame_size_limit(), 1920 * 1080);
        assert!(!config.settings.get_strict_std_compliance());
        assert!(config.settings.get_output_invisible_frames());
        assert_eq!(
            config.settings.get_inloop_filters(),
            all_inloop_filters() & !InloopFilterType::DEBLOCK
        );
        assert_eq!(
            config.settings.get_decode_frame_type(),
            DecodeFrameType::Key
        );
    }

    #[test]
    fn rejects_unknown_values() {
        assert!(
            parse(&["-i", "in.ivf", "-o", "out.y4m", "--inloopfilters", "nope"])
                .resolve()
                .is_err()
        );
        assert!(
            parse(&["-i", "in.ivf", "-o", "out.y4m", "--sizelimit", "0x0x0"])
                .resolve()
                .is_err()
        );
        assert!(parse(&["-i", "in.ivf", "-o", "out.unknown"])
            .resolve()
            .is_err());
    }

    #[test]
    fn detects_per_frame_patterns() {
        assert!(has_per_frame_pattern(Path::new("out%03n.y4m")));
        assert!(!has_per_frame_pattern(Path::new("%w-%h.yuv")));
        assert!(!has_per_frame_pattern(Path::new("out.y4m")));
        assert!(!has_per_frame_pattern(Path::new("out%q.y4m")));
    }
}
