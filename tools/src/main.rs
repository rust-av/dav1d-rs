mod ivf {
    use bitstream_io::{BitReader, LittleEndian};
    use std::io;

    #[derive(Debug, PartialEq)]
    pub struct Header {
        pub tag: [u8; 4],
        pub w: u16,
        pub h: u16,
        pub timebase_num: u32,
        pub timebase_den: u32,
    }

    pub fn read_header(r: &mut dyn io::Read) -> io::Result<Header> {
        let mut br = BitReader::endian(r, LittleEndian);

        let mut signature = [0u8; 4];
        let mut tag = [0u8; 4];

        br.read_bytes(&mut signature)?;
        let v0: u16 = br.read(16)?;
        let v1: u16 = br.read(16)?;
        br.read_bytes(&mut tag)?;

        log::debug!("sign {:?} version {} {} tag {:?}", &signature, v0, v1, &tag);

        let w: u16 = br.read(16)?;
        let h: u16 = br.read(16)?;

        let timebase_den: u32 = br.read(32)?;
        let timebase_num: u32 = br.read(32)?;

        let _: u32 = br.read(32)?;
        let _: u32 = br.read(32)?;

        Ok(Header {
            tag,
            w,
            h,
            timebase_num,
            timebase_den,
        })
    }

    pub struct Packet {
        pub data: Box<[u8]>,
        pub pts: u64,
    }

    pub fn read_packet(r: &mut dyn io::Read) -> io::Result<Packet> {
        let mut br = BitReader::endian(r, LittleEndian);

        let len: u32 = br.read(32)?;
        let pts: u64 = br.read(64)?;
        let mut buf = vec![0u8; len as usize];

        br.read_bytes(&mut buf)?;

        Ok(Packet {
            data: buf.into_boxed_slice(),
            pts,
        })
    }
}

use structopt::*;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    input: std::path::PathBuf,
}

use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let file = File::open(opt.input)?;
    let mut r = BufReader::new(file);
    let header = ivf::read_header(&mut r)?;
    println!("{:?}", header);

    let mut dec = dav1d::Decoder::new();

    while let Ok(packet) = ivf::read_packet(&mut r) {
        println!("Packet {}", packet.pts);
        dec.send_data(packet.data);
        loop {
            match dec.get_picture() {
                Ok(p) => unsafe {
                    let frame_hdr = *p.frame_hdr;
                    println!(
                        "  Frame {} {} {}",
                        frame_hdr.frame_id, frame_hdr.show_frame, frame_hdr.showable_frame
                    );
                },
                Err(e) => {
                    if e == -(dav1d::EAGAIN as i32) {
                        break;
                    } else {
                        panic!("Error {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
