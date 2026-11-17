//! Muxers mirroring `dav1d`'s `yuv4mpeg2`, `yuv`, `md5` and `null` ones.

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use dav1d::{dav1d_sys, Picture, PixelLayout, PlanarImageComponent};

/// The output formats `dav1d` can write.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MuxerKind {
    Null,
    Md5,
    Yuv,
    Y4m,
}

impl MuxerKind {
    pub fn from_name(name: &str) -> Option<MuxerKind> {
        match name {
            "null" => Some(MuxerKind::Null),
            "md5" => Some(MuxerKind::Md5),
            "yuv" => Some(MuxerKind::Yuv),
            "yuv4mpeg2" => Some(MuxerKind::Y4m),
            _ => None,
        }
    }

    pub fn from_path(path: &Path) -> Option<MuxerKind> {
        let extension = path.extension()?.to_str()?;
        match extension {
            "y4m" => Some(MuxerKind::Y4m),
            "yuv" => Some(MuxerKind::Yuv),
            "md5" => Some(MuxerKind::Md5),
            _ => None,
        }
    }
}

/// Writes decoded frames in the selected format.
pub struct Output {
    kind: MuxerKind,
    path: Option<PathBuf>,
    per_frame: bool,
    fps: (u32, u32),
    framenum: usize,
    writer: Option<Box<dyn Write>>,
    md5: md5::Context,
    header_written: bool,
}

impl Output {
    pub fn new(
        kind: MuxerKind,
        path: Option<PathBuf>,
        per_frame: bool,
        fps: (u32, u32),
    ) -> io::Result<Self> {
        let writer = if per_frame || kind == MuxerKind::Null {
            None
        } else {
            Some(open_writer(path.as_deref())?)
        };

        Ok(Self {
            kind,
            path,
            per_frame,
            fps,
            framenum: 0,
            writer,
            md5: md5::Context::new(),
            header_written: false,
        })
    }

    /// Writes one decoded frame.
    pub fn write_frame(&mut self, picture: &Picture) -> io::Result<()> {
        match self.kind {
            MuxerKind::Null => Ok(()),
            MuxerKind::Md5 => {
                if self.per_frame {
                    let mut writer = self.open_frame_writer(picture)?;
                    let mut md5 = md5::Context::new();
                    hash_picture(&mut md5, picture)?;
                    writeln!(writer, "{:x}", md5.finalize())?;
                    writer.flush()
                } else {
                    hash_picture(&mut self.md5, picture)
                }
            }
            MuxerKind::Yuv => {
                if self.per_frame {
                    let mut writer = self.open_frame_writer(picture)?;
                    write_yuv(&mut writer, picture)?;
                    writer.flush()
                } else {
                    let writer = self.writer.as_mut().expect("opened for yuv output");
                    write_yuv(writer, picture)
                }
            }
            MuxerKind::Y4m => {
                if self.per_frame {
                    let mut writer = self.open_frame_writer(picture)?;
                    write_y4m_header(&mut writer, picture, self.fps)?;
                    write_y4m_frame(&mut writer, picture)?;
                    writer.flush()
                } else {
                    let writer = self.writer.as_mut().expect("opened for y4m output");
                    if !self.header_written {
                        self.header_written = true;
                        write_y4m_header(writer, picture, self.fps)?;
                    }
                    write_y4m_frame(writer, picture)
                }
            }
        }
    }

    /// Flushes the output, returning the md5 of the decoded frames if any.
    ///
    /// `write_digest` says whether the hexadecimal digest should also be
    /// written to the output; `--verify` does not print it.
    pub fn finish(&mut self, write_digest: bool) -> io::Result<Option<String>> {
        let digest = if self.kind == MuxerKind::Md5 && !self.per_frame {
            let digest = format!("{:x}", self.md5.clone().finalize());
            if write_digest {
                if let Some(writer) = self.writer.as_mut() {
                    writeln!(writer, "{digest}")?;
                }
            }
            Some(digest)
        } else {
            None
        };

        if let Some(writer) = self.writer.as_mut() {
            writer.flush()?;
        }

        Ok(digest)
    }

    fn open_frame_writer(&mut self, picture: &Picture) -> io::Result<Box<dyn Write>> {
        let pattern = self.path.as_deref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Per-frame output requires an output file",
            )
        })?;
        let filename = assemble_filename(pattern, self.framenum, picture.width(), picture.height());
        self.framenum += 1;
        open_writer(Some(&filename))
    }
}

fn open_writer(path: Option<&Path>) -> io::Result<Box<dyn Write>> {
    match path {
        Some(path) if path != Path::new("-") => Ok(Box::new(BufWriter::new(File::create(path)?))),
        _ => Ok(Box::new(BufWriter::new(io::stdout()))),
    }
}

/// Expands `%n`, `%w` and `%h` in a per-frame output pattern.
///
/// `%03n` pads with zeroes to three digits, as `dav1d` does.
pub fn assemble_filename(pattern: &Path, frame: usize, width: u32, height: u32) -> PathBuf {
    let Some(pattern) = pattern.to_str() else {
        return pattern.to_path_buf();
    };
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::with_capacity(pattern.len() + 8);
    let mut i = 0;

    while i < chars.len() {
        if chars[i] != '%' {
            out.push(chars[i]);
            i += 1;
            continue;
        }

        i += 1;
        let digits_start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        let padding: usize = chars[digits_start..i]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap_or(0);

        let field = match chars.get(i) {
            Some('n') => Some(frame as u64),
            Some('w') => Some(width as u64),
            Some('h') => Some(height as u64),
            _ => None,
        };

        if let Some(field) = field {
            if padding == 0 {
                out.push_str(&field.to_string());
            } else {
                out.push_str(&format!("{field:0padding$}"));
            }
            i += 1;
        } else {
            // Not a pattern: emit the `%` and rescan what follows.
            out.push('%');
            i = digits_start;
        }
    }

    PathBuf::from(out)
}

fn chroma_position(picture: &Picture) -> dav1d_sys::Dav1dChromaSamplePosition {
    let picture = picture.dav1d_picture();
    if picture.seq_hdr.is_null() {
        return dav1d_sys::DAV1D_CHR_UNKNOWN;
    }
    unsafe { (*picture.seq_hdr).chr }
}

fn bits_per_component(picture: &Picture) -> usize {
    picture.bits_per_component().map(|bits| bits.0).unwrap_or(8)
}

fn render_dimensions(picture: &Picture) -> (u32, u32) {
    let raw = picture.dav1d_picture();
    if raw.frame_hdr.is_null() {
        return (picture.width(), picture.height());
    }
    let header = unsafe { &*raw.frame_hdr };
    if header.render_width > 0 && header.render_height > 0 {
        (header.render_width as u32, header.render_height as u32)
    } else {
        (picture.width(), picture.height())
    }
}

fn y4m_tag(picture: &Picture) -> &'static str {
    let layout = picture.pixel_layout();
    let bits = bits_per_component(picture);

    if layout == PixelLayout::I420 && bits <= 8 {
        return match chroma_position(picture) {
            dav1d_sys::DAV1D_CHR_VERTICAL => "420mpeg2",
            dav1d_sys::DAV1D_CHR_COLOCATED => "420",
            _ => "420jpeg",
        };
    }

    let names = match layout {
        PixelLayout::I400 => ["mono", "mono10", "mono12"],
        PixelLayout::I420 => ["420jpeg", "420p10", "420p12"],
        PixelLayout::I422 => ["422", "422p10", "422p12"],
        PixelLayout::I444 => ["444", "444p10", "444p12"],
    };
    names[match bits {
        0..=8 => 0,
        9..=10 => 1,
        _ => 2,
    }]
}

fn aspect_ratio(picture: &Picture) -> (u64, u64) {
    let (render_width, render_height) = render_dimensions(picture);
    let mut aw = u64::from(picture.height()) * u64::from(render_width);
    let mut ah = u64::from(picture.width()) * u64::from(render_height);

    if aw == 0 || ah == 0 {
        return (1, 1);
    }

    let mut a = aw;
    let mut b = ah;
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    aw /= a;
    ah /= a;

    (aw, ah)
}

fn write_y4m_header<W: Write>(
    writer: &mut W,
    picture: &Picture,
    fps: (u32, u32),
) -> io::Result<()> {
    let (aw, ah) = aspect_ratio(picture);
    write!(
        writer,
        "YUV4MPEG2 W{} H{} F{}:{} Ip A{}:{} C{}",
        picture.width(),
        picture.height(),
        fps.0,
        fps.1,
        aw,
        ah,
        y4m_tag(picture),
    )?;
    writer.write_all(b"\n")
}

fn write_y4m_frame<W: Write>(writer: &mut W, picture: &Picture) -> io::Result<()> {
    writer.write_all(b"FRAME\n")?;
    write_yuv(writer, picture)
}

fn write_yuv<W: Write>(writer: &mut W, picture: &Picture) -> io::Result<()> {
    for_each_row(picture, |row| writer.write_all(row))
}

fn hash_picture(md5: &mut md5::Context, picture: &Picture) -> io::Result<()> {
    let high_bitdepth = picture.bit_depth() > 8;

    for_each_row(picture, |row| {
        if high_bitdepth {
            // The checksum is defined over little-endian samples.
            let mut le = Vec::with_capacity(row.len());
            for sample in row.chunks(2) {
                if sample.len() == 2 {
                    le.extend_from_slice(&u16::from_ne_bytes([sample[0], sample[1]]).to_le_bytes());
                }
            }
            md5.consume(&le);
        } else {
            md5.consume(row);
        }
        Ok(())
    })
}

/// Calls `f` with the visible bytes of every plane row, in Y, U, V order.
fn for_each_row<F>(picture: &Picture, mut f: F) -> io::Result<()>
where
    F: FnMut(&[u8]) -> io::Result<()>,
{
    let layout = picture.pixel_layout();
    let bytes_per_sample = if picture.bit_depth() > 8 { 2 } else { 1 };
    let ss_hor = match layout {
        PixelLayout::I400 | PixelLayout::I444 | PixelLayout::I422 => 0,
        PixelLayout::I420 => 1,
    };

    let components: &[PlanarImageComponent] = if layout == PixelLayout::I400 {
        &[PlanarImageComponent::Y]
    } else {
        &[
            PlanarImageComponent::Y,
            PlanarImageComponent::U,
            PlanarImageComponent::V,
        ]
    };

    for &component in components {
        let width = if component == PlanarImageComponent::Y {
            picture.width()
        } else {
            (picture.width() + ss_hor) >> ss_hor
        };
        let (stride, height) = picture.plane_data_geometry(component);

        if stride == 0 || height == 0 || width == 0 {
            continue;
        }

        let row_len = width as usize * bytes_per_sample;
        let stride = stride as usize;
        if stride < row_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Plane stride is smaller than the plane width",
            ));
        }

        let plane = picture.plane(component);
        let plane: &[u8] = plane.as_ref();

        for row in 0..height as usize {
            let start = row * stride;
            let row = plane.get(start..start + row_len).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Plane is shorter than expected",
                )
            })?;
            f(row)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_frame_patterns() {
        assert_eq!(
            assemble_filename(Path::new("out%03n.y4m"), 7, 320, 240),
            PathBuf::from("out007.y4m")
        );
        assert_eq!(
            assemble_filename(Path::new("%w-%h.yuv"), 0, 1920, 1080),
            PathBuf::from("1920-1080.yuv")
        );
        assert_eq!(
            assemble_filename(Path::new("frame%n_%w_%h.md5"), 12, 16, 8),
            PathBuf::from("frame12_16_8.md5")
        );
    }

    #[test]
    fn leaves_unknown_patterns_alone() {
        assert_eq!(
            assemble_filename(Path::new("out%q.y4m"), 1, 2, 3),
            PathBuf::from("out%q.y4m")
        );
        assert_eq!(
            assemble_filename(Path::new("out%%n.y4m"), 1, 2, 3),
            PathBuf::from("out%1.y4m")
        );
    }

    #[test]
    fn finds_muxers_by_name_and_extension() {
        assert_eq!(MuxerKind::from_name("yuv4mpeg2"), Some(MuxerKind::Y4m));
        assert_eq!(MuxerKind::from_name("y4m"), None);
        assert_eq!(MuxerKind::from_name("xxh3"), None);
        assert_eq!(
            MuxerKind::from_path(Path::new("a/b.y4m")),
            Some(MuxerKind::Y4m)
        );
        assert_eq!(
            MuxerKind::from_path(Path::new("a/b.yuv")),
            Some(MuxerKind::Yuv)
        );
        assert_eq!(
            MuxerKind::from_path(Path::new("a/b.md5")),
            Some(MuxerKind::Md5)
        );
        assert_eq!(MuxerKind::from_path(Path::new("a/b.mkv")), None);
    }
}
