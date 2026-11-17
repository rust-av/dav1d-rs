//! Demuxers mirroring `dav1d`'s `ivf`, `annexb` and `section5` ones.

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// A packet of AV1 data, with the metadata coming from the container.
pub struct Packet {
    pub data: Vec<u8>,
    pub offset: Option<i64>,
    pub timestamp: Option<i64>,
}

/// What a [`Demuxer`] knows about the stream before decoding starts.
pub trait Demuxer {
    /// Frame rate as a `(numerator, denominator)` pair, `(0, 0)` if unknown.
    fn fps(&self) -> (u32, u32);
    /// Number of packets in the stream, if known.
    fn num_frames(&self) -> Option<u32>;
    /// Reads the next packet, `Ok(None)` at the end of the stream.
    fn read_packet(&mut self) -> io::Result<Option<Packet>>;
}

/// The container formats `dav1d` can read.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Ivf,
    Annexb,
    Section5,
}

impl Kind {
    pub fn from_name(name: &str) -> Option<Kind> {
        match name {
            "ivf" => Some(Kind::Ivf),
            "annexb" => Some(Kind::Annexb),
            "section5" => Some(Kind::Section5),
            _ => None,
        }
    }
}

/// Opens `path` with the demuxer named by `forced`, or detecting it.
pub fn open(path: &Path, forced: Option<Kind>) -> io::Result<Box<dyn Demuxer>> {
    let kind = match forced {
        Some(kind) => kind,
        None => detect(path)?,
    };
    let file = BufReader::new(File::open(path)?);
    Ok(match kind {
        Kind::Ivf => Box::new(Ivf::new(file)?),
        Kind::Annexb => Box::new(Annexb::new(file)),
        Kind::Section5 => Box::new(Section5::new(file)),
    })
}

const PROBE_SIZE: usize = 2048;

/// Detects the container format from the first bytes of the file.
pub fn detect(path: &Path) -> io::Result<Kind> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; PROBE_SIZE];
    let mut len = 0;
    while len < buf.len() {
        match file.read(&mut buf[len..])? {
            0 => break,
            n => len += n,
        }
    }
    let buf = &buf[..len];

    if buf.starts_with(b"DKIF") {
        Ok(Kind::Ivf)
    } else if annexb_probe(buf) {
        Ok(Kind::Annexb)
    } else if section5_probe(buf) {
        Ok(Kind::Section5)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to detect the demuxer of {}", path.display()),
        ))
    }
}

// OBU types, from the AV1 specification.
const OBU_SEQ_HDR: u8 = 1;
const OBU_TD: u8 = 2;
const OBU_FRAME_HDR: u8 = 3;
const OBU_TILE_GRP: u8 = 4;
const OBU_FRAME: u8 = 6;

/// The header of an AV1 open bitstream unit.
#[derive(Debug, PartialEq, Eq)]
pub struct ObuHeader {
    pub obu_type: u8,
    /// Size of the payload, excluding the header and the size field.
    pub payload_size: usize,
    /// Size of the whole OBU, header and size field included.
    pub size: usize,
}

/// Parses an OBU header, mirroring `dav1d`'s `parse_obu_header()`.
pub fn parse_obu_header(buf: &[u8], allow_implicit_size: bool) -> Option<ObuHeader> {
    let b0 = *buf.first()?;
    if b0 & 0x80 != 0 {
        // obu_forbidden_bit
        return None;
    }
    let obu_type = (b0 & 0x78) >> 3;
    let extension_flag = (b0 & 0x04) >> 2;
    let has_size_flag = (b0 & 0x02) >> 1;
    let mut pos = 1 + extension_flag as usize;
    if pos > buf.len() {
        return None;
    }

    if has_size_flag != 0 {
        let (payload_size, leb_len) = leb128_slice(&buf[pos..])?;
        pos += leb_len;
        if payload_size > buf.len() - pos {
            return None;
        }
        Some(ObuHeader {
            obu_type,
            payload_size,
            size: pos + payload_size,
        })
    } else if allow_implicit_size {
        Some(ObuHeader {
            obu_type,
            payload_size: buf.len() - pos,
            size: buf.len(),
        })
    } else {
        None
    }
}

/// Decodes a LEB128 value, returning it and the number of bytes it occupied.
pub fn leb128_slice(buf: &[u8]) -> Option<(usize, usize)> {
    let mut value = 0u64;
    for (i, &byte) in buf.iter().take(8).enumerate() {
        value |= u64::from(byte & 0x7f) << (i * 7);
        if byte & 0x80 == 0 {
            if value > u32::MAX as u64 {
                return None;
            }
            return Some((value as usize, i + 1));
        }
    }
    None
}

/// Reads a LEB128 value from `r`, returning it and the number of bytes read.
///
/// `Ok(None)` means the stream ended before the first byte.
fn read_leb128<R: Read>(r: &mut R) -> io::Result<Option<(usize, usize)>> {
    let mut value = 0u64;
    for i in 0..8 {
        let mut byte = [0u8; 1];
        match r.read(&mut byte) {
            Ok(0) if i == 0 => return Ok(None),
            Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
        value |= u64::from(byte[0] & 0x7f) << (i * 7);
        if byte[0] & 0x80 == 0 {
            if value > u32::MAX as u64 {
                return Err(io::ErrorKind::InvalidData.into());
            }
            return Ok(Some((value as usize, i + 1)));
        }
    }
    Err(io::ErrorKind::InvalidData.into())
}

fn annexb_probe(buf: &[u8]) -> bool {
    let mut pos = 0;

    let Some((temporal_unit_size, temporal_header)) = leb128_slice(&buf[pos..]) else {
        return false;
    };
    pos += temporal_header;

    let Some((frame_unit_size, frame_header)) = leb128_slice(&buf[pos..]) else {
        return false;
    };
    if frame_unit_size as u64 + frame_header as u64 > temporal_unit_size as u64 {
        return false;
    }
    pos += frame_header;

    let mut temporal_left = temporal_unit_size as i64 - frame_header as i64;
    let mut frame_left = frame_unit_size as i64;

    // The first OBU has to be a Temporal Delimiter without a payload.
    let Some((obu_size, obu_header)) = leb128_slice(&buf[pos..]) else {
        return false;
    };
    if obu_size as i64 + obu_header as i64 >= frame_left {
        return false;
    }
    temporal_left -= obu_size as i64 + obu_header as i64;
    frame_left -= obu_size as i64 + obu_header as i64;
    pos += obu_header;

    let Some(header) = parse_obu_header(&buf[pos..], true) else {
        return false;
    };
    if header.obu_type != OBU_TD || header.payload_size != 0 {
        return false;
    }
    pos += obu_size;

    // Then look for the first frame and its sequence header.
    let mut seq = false;
    while pos < buf.len() {
        let Some((obu_size, obu_header)) = leb128_slice(&buf[pos..]) else {
            return false;
        };
        if obu_size as i64 + obu_header as i64 > frame_left {
            return false;
        }
        temporal_left -= obu_header as i64;
        frame_left -= obu_header as i64;
        pos += obu_header;

        let Some(header) = parse_obu_header(&buf[pos..], true) else {
            return false;
        };
        pos += obu_size;

        match header.obu_type {
            OBU_SEQ_HDR => seq = true,
            OBU_FRAME | OBU_FRAME_HDR => return seq,
            OBU_TD | OBU_TILE_GRP => return false,
            _ => {}
        }

        temporal_left -= obu_size as i64;
        frame_left -= obu_size as i64;
        if frame_left <= 0 || temporal_left < 0 {
            return false;
        }
    }

    seq
}

fn section5_probe(buf: &[u8]) -> bool {
    let mut pos = 0;

    // The first OBU has to be a Temporal Delimiter without a payload.
    let Some(header) = parse_obu_header(&buf[pos..], false) else {
        return false;
    };
    if header.obu_type != OBU_TD || header.payload_size != 0 {
        return false;
    }
    pos += header.size;

    let mut seq = false;
    while pos < buf.len() {
        let Some(header) = parse_obu_header(&buf[pos..], false) else {
            return false;
        };
        pos += header.size;

        match header.obu_type {
            OBU_SEQ_HDR => seq = true,
            OBU_FRAME | OBU_FRAME_HDR => return seq,
            OBU_TD | OBU_TILE_GRP => return false,
            _ => {}
        }
    }

    seq
}

const IVF_HEADER_SIZE: usize = 32;

/// The fields of an IVF file header that matter here.
#[derive(Debug, PartialEq, Eq)]
pub struct IvfHeader {
    pub width: u16,
    pub height: u16,
    pub rate: u32,
    pub scale: u32,
    pub num_frames: u32,
}

/// Parses a 32 bytes IVF header.
pub fn parse_ivf_header(buf: &[u8]) -> io::Result<IvfHeader> {
    if buf.len() < IVF_HEADER_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to read stream header",
        ));
    }
    if &buf[0..4] != b"DKIF" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("not an IVF file [tag={:?}]", &buf[0..4]),
        ));
    }
    if &buf[8..12] != b"AV01" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("not an AV1 file [tag={:?}]", &buf[8..12]),
        ));
    }

    let u16_at = |offset: usize| u16::from_le_bytes([buf[offset], buf[offset + 1]]);
    let u32_at = |offset: usize| {
        u32::from_le_bytes([
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ])
    };

    Ok(IvfHeader {
        width: u16_at(12),
        height: u16_at(14),
        rate: u32_at(16),
        scale: u32_at(20),
        num_frames: u32_at(24),
    })
}

fn reduce(mut num: u64, mut den: u64) -> (u32, u32) {
    if num == 0 || den == 0 {
        return (0, 0);
    }
    let mut a = num;
    let mut b = den;
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    let gcd = a;
    num /= gcd;
    den /= gcd;
    while (num | den) > u32::MAX as u64 {
        num >>= 1;
        den >>= 1;
    }
    (num as u32, den as u32)
}

/// An IVF (`DKIF`) demuxer.
pub struct Ivf<R: Read> {
    r: R,
    header: IvfHeader,
    fps: (u32, u32),
    offset: u64,
    last_ts: u64,
    broken: bool,
}

impl<R: Read> Ivf<R> {
    pub fn new(mut r: R) -> io::Result<Self> {
        let mut buf = [0u8; IVF_HEADER_SIZE];
        r.read_exact(&mut buf)?;
        let header = parse_ivf_header(&buf)?;
        let fps = reduce(header.rate as u64, header.scale as u64);
        Ok(Self {
            r,
            header,
            fps,
            offset: IVF_HEADER_SIZE as u64,
            last_ts: 0,
            broken: false,
        })
    }
}

impl<R: Read> Demuxer for Ivf<R> {
    fn fps(&self) -> (u32, u32) {
        self.fps
    }

    fn num_frames(&self) -> Option<u32> {
        Some(self.header.num_frames).filter(|n| *n != 0)
    }

    fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        let mut size = [0u8; 4];
        match self.r.read_exact(&mut size) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }
        let size = u32::from_le_bytes(size) as usize;

        let mut ts = [0u8; 8];
        self.r.read_exact(&mut ts)?;
        let stored_ts = u64::from_le_bytes(ts);

        // Streams written by some muxers repeat timestamps, in which case
        // `dav1d` synthesizes them from the frame index.
        if self.offset > IVF_HEADER_SIZE as u64 && stored_ts <= self.last_ts {
            self.broken = true;
        }
        let timestamp = if self.broken {
            self.last_ts + 1
        } else {
            stored_ts
        };
        self.last_ts = timestamp;

        let mut data = vec![0u8; size];
        self.r.read_exact(&mut data)?;
        let offset = self.offset as i64;
        self.offset += 4 + 8 + size as u64;

        Ok(Some(Packet {
            data,
            offset: Some(offset),
            timestamp: Some(timestamp as i64),
        }))
    }
}

/// An Annex B (low overhead bitstream format) demuxer.
///
/// Every packet it outputs is a single OBU, as `dav1d`'s demuxer does.
pub struct Annexb<R: Read> {
    r: R,
    temporal_unit_size: usize,
    frame_unit_size: usize,
}

impl<R: Read> Annexb<R> {
    pub fn new(r: R) -> Self {
        Self {
            r,
            temporal_unit_size: 0,
            frame_unit_size: 0,
        }
    }
}

impl<R: Read> Demuxer for Annexb<R> {
    fn fps(&self) -> (u32, u32) {
        // The timing information is in the sequence header, if any.
        (25, 1)
    }

    fn num_frames(&self) -> Option<u32> {
        None
    }

    fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        if self.temporal_unit_size == 0 {
            match read_leb128(&mut self.r)? {
                Some((size, _)) => self.temporal_unit_size = size,
                None => return Ok(None),
            }
        }
        if self.frame_unit_size == 0 {
            let (size, size_len) = read_leb128(&mut self.r)?.ok_or(io::ErrorKind::UnexpectedEof)?;
            if size + size_len > self.temporal_unit_size {
                return Err(io::ErrorKind::InvalidData.into());
            }
            self.temporal_unit_size -= size_len;
            self.frame_unit_size = size;
        }

        let (size, size_len) = read_leb128(&mut self.r)?.ok_or(io::ErrorKind::UnexpectedEof)?;
        if size + size_len > self.frame_unit_size {
            return Err(io::ErrorKind::InvalidData.into());
        }
        self.temporal_unit_size -= size + size_len;
        self.frame_unit_size -= size + size_len;

        let mut data = vec![0u8; size];
        self.r.read_exact(&mut data)?;
        Ok(Some(Packet {
            data,
            offset: None,
            timestamp: None,
        }))
    }
}

/// A Section 5 (one temporal unit per packet) demuxer.
pub struct Section5<R: Read> {
    r: R,
    pushback: Option<u8>,
}

impl<R: Read> Section5<R> {
    pub fn new(r: R) -> Self {
        Self { r, pushback: None }
    }

    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        if let Some(byte) = self.pushback.take() {
            return Ok(Some(byte));
        }
        let mut byte = [0u8; 1];
        match self.r.read(&mut byte) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(byte[0])),
            Err(e) => Err(e),
        }
    }

    fn read_exact_into(&mut self, buf: &mut Vec<u8>, len: usize) -> io::Result<()> {
        let start = buf.len();
        buf.resize(start + len, 0);
        self.r.read_exact(&mut buf[start..])
    }

    fn read_leb128(&mut self) -> io::Result<Option<(usize, Vec<u8>)>> {
        let mut value = 0u64;
        let mut bytes = Vec::new();
        for i in 0..8 {
            let Some(byte) = self.read_byte()? else {
                if i == 0 {
                    return Ok(None);
                }
                return Err(io::ErrorKind::UnexpectedEof.into());
            };
            bytes.push(byte);
            value |= u64::from(byte & 0x7f) << (i * 7);
            if byte & 0x80 == 0 {
                if value > u32::MAX as u64 {
                    return Err(io::ErrorKind::InvalidData.into());
                }
                return Ok(Some((value as usize, bytes)));
            }
        }
        Err(io::ErrorKind::InvalidData.into())
    }
}

impl<R: Read> Demuxer for Section5<R> {
    fn fps(&self) -> (u32, u32) {
        (25, 1)
    }

    fn num_frames(&self) -> Option<u32> {
        None
    }

    fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        let mut data = Vec::new();

        loop {
            let first = data.is_empty();
            let Some(byte) = self.read_byte()? else {
                if first {
                    return Ok(None);
                }
                break;
            };

            let obu_type = (byte >> 3) & 0xf;
            if first {
                if obu_type != OBU_TD {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "packet does not start with a temporal delimiter",
                    ));
                }
            } else if obu_type == OBU_TD {
                // The temporal delimiter starts the next packet.
                self.pushback = Some(byte);
                break;
            }

            let has_size_field = byte & 0x02 != 0;
            if !has_size_field {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "OBU without a size field",
                ));
            }
            let has_extension = byte & 0x04 != 0;
            data.push(byte);
            if has_extension {
                let Some(extension) = self.read_byte()? else {
                    return Err(io::ErrorKind::UnexpectedEof.into());
                };
                data.push(extension);
            }
            let Some((size, leb)) = self.read_leb128()? else {
                return Err(io::ErrorKind::UnexpectedEof.into());
            };
            data.extend_from_slice(&leb);
            self.read_exact_into(&mut data, size)?;
        }

        Ok(Some(Packet {
            data,
            offset: None,
            timestamp: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn leb(mut value: usize) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                out.push(byte);
                return out;
            }
            out.push(byte | 0x80);
        }
    }

    const TD: &[u8] = &[0x12, 0x00]; // Temporal delimiter, empty payload.
    const SEQ: &[u8] = &[0x0a, 0x01, 0xaa]; // Sequence header, 1 byte payload.
    const FRAME: &[u8] = &[0x32, 0x02, 0xbb, 0xcc]; // Frame, 2 bytes payload.

    fn ivf_header(num_frames: u32) -> Vec<u8> {
        let mut hdr = Vec::new();
        hdr.extend_from_slice(b"DKIF");
        hdr.extend_from_slice(&0u16.to_le_bytes());
        hdr.extend_from_slice(&32u16.to_le_bytes());
        hdr.extend_from_slice(b"AV01");
        hdr.extend_from_slice(&320u16.to_le_bytes());
        hdr.extend_from_slice(&240u16.to_le_bytes());
        hdr.extend_from_slice(&30u32.to_le_bytes());
        hdr.extend_from_slice(&1u32.to_le_bytes());
        hdr.extend_from_slice(&num_frames.to_le_bytes());
        hdr.extend_from_slice(&0u32.to_le_bytes());
        hdr
    }

    #[test]
    fn parses_ivf_headers() {
        let header = parse_ivf_header(&ivf_header(2)).unwrap();
        assert_eq!(
            header,
            IvfHeader {
                width: 320,
                height: 240,
                rate: 30,
                scale: 1,
                num_frames: 2,
            }
        );
        assert!(parse_ivf_header(&ivf_header(2)[..16]).is_err());

        let mut wrong_tag = ivf_header(1);
        wrong_tag[8..12].copy_from_slice(b"VP80");
        assert!(parse_ivf_header(&wrong_tag).is_err());
    }

    #[test]
    fn reads_ivf_packets() {
        let mut stream = ivf_header(2);
        for (i, data) in [&b"one"[..], &b"two"[..]].iter().enumerate() {
            stream.extend_from_slice(&(data.len() as u32).to_le_bytes());
            stream.extend_from_slice(&((i as u64) * 10).to_le_bytes());
            stream.extend_from_slice(data);
        }

        let mut ivf = Ivf::new(Cursor::new(stream)).unwrap();
        assert_eq!(ivf.fps(), (30, 1));
        assert_eq!(ivf.num_frames(), Some(2));

        let packet = ivf.read_packet().unwrap().unwrap();
        assert_eq!(packet.data, b"one");
        assert_eq!(packet.timestamp, Some(0));
        let packet = ivf.read_packet().unwrap().unwrap();
        assert_eq!(packet.data, b"two");
        assert_eq!(packet.timestamp, Some(10));
        assert!(ivf.read_packet().unwrap().is_none());
    }

    #[test]
    fn synthesizes_repeated_ivf_timestamps() {
        let mut stream = ivf_header(2);
        for data in [&b"one"[..], &b"two"[..]] {
            stream.extend_from_slice(&(data.len() as u32).to_le_bytes());
            stream.extend_from_slice(&7u64.to_le_bytes());
            stream.extend_from_slice(data);
        }

        let mut ivf = Ivf::new(Cursor::new(stream)).unwrap();
        assert_eq!(ivf.read_packet().unwrap().unwrap().timestamp, Some(7));
        assert_eq!(ivf.read_packet().unwrap().unwrap().timestamp, Some(8));
    }

    #[test]
    fn parses_obu_headers() {
        let header = parse_obu_header(SEQ, false).unwrap();
        assert_eq!(header.obu_type, OBU_SEQ_HDR);
        assert_eq!(header.payload_size, 1);
        assert_eq!(header.size, 3);

        // Without a size field only the Annex B path accepts the OBU.
        assert!(parse_obu_header(&[0x08, 0x01], false).is_none());
        let header = parse_obu_header(&[0x08, 0x01], true).unwrap();
        assert_eq!(header.payload_size, 1);

        assert!(parse_obu_header(&[0x80], false).is_none());
    }

    #[test]
    fn decodes_leb128() {
        assert_eq!(leb128_slice(&[0x00]), Some((0, 1)));
        assert_eq!(leb128_slice(&[0x7f]), Some((127, 1)));
        assert_eq!(leb128_slice(&[0x80, 0x01]), Some((128, 2)));
        assert_eq!(leb128_slice(&[0x80]), None);
        assert_eq!(leb128_slice(&leb(300)), Some((300, 2)));
    }

    #[test]
    fn reads_annexb_packets() {
        let mut frame_unit = Vec::new();
        for obu in [TD, SEQ, FRAME] {
            frame_unit.extend_from_slice(&leb(obu.len()));
            frame_unit.extend_from_slice(obu);
        }

        // The temporal unit holds the frame unit size and the frame unit.
        let frame_unit_size = frame_unit.len();
        let temporal_unit_size = leb(frame_unit_size).len() + frame_unit_size;
        let mut stream = leb(temporal_unit_size);
        stream.extend_from_slice(&leb(frame_unit_size));
        stream.extend_from_slice(&frame_unit);

        assert!(annexb_probe(&stream));

        let mut annexb = Annexb::new(Cursor::new(stream));
        let expected = [TD, SEQ, FRAME];
        for obu in expected {
            assert_eq!(annexb.read_packet().unwrap().unwrap().data, obu);
        }
        assert!(annexb.read_packet().unwrap().is_none());
    }

    #[test]
    fn reads_section5_packets() {
        let mut stream = Vec::new();
        for obu in [TD, SEQ, FRAME, TD, FRAME] {
            stream.extend_from_slice(obu);
        }

        assert!(section5_probe(&stream));
        assert!(!section5_probe(&stream[1..]));

        let mut section5 = Section5::new(Cursor::new(stream));
        let mut expected = Vec::new();
        for obu in [TD, SEQ, FRAME] {
            expected.extend_from_slice(obu);
        }
        assert_eq!(section5.read_packet().unwrap().unwrap().data, expected);

        let mut expected = Vec::new();
        for obu in [TD, FRAME] {
            expected.extend_from_slice(obu);
        }
        assert_eq!(section5.read_packet().unwrap().unwrap().data, expected);
        assert!(section5.read_packet().unwrap().is_none());
    }

    #[test]
    fn section5_rejects_packets_without_temporal_delimiter() {
        let stream = [SEQ, FRAME].concat();
        let mut section5 = Section5::new(Cursor::new(stream));
        assert!(section5.read_packet().is_err());
    }
}
