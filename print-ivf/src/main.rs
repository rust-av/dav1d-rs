mod ivf {
    use bitstream_io::{BitRead, BitReader, LittleEndian};
    use std::io;

    #[derive(Debug, PartialEq, Eq)]
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

fn handle_pending_pictures(dec: &mut dav1d::Decoder, drain: bool) {
    loop {
        match dec.get_picture() {
            Ok(p) => println!("{p:?}"),
            // Need to send more data to the decoder before it can decode new pictures
            Err(e) if e.is_again() => return,
            Err(e) => {
                panic!("Error getting decoded pictures: {}", e);
            }
        }

        if !drain {
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let file = File::open(opt.input)?;
    let mut r = BufReader::new(file);
    let header = ivf::read_header(&mut r)?;
    println!("{:?}", header);

    let mut dec = dav1d::Decoder::new().expect("failed to create decoder instance");

    while let Ok(packet) = ivf::read_packet(&mut r) {
        println!("Packet {}", packet.pts);

        // Send packet to the decoder
        match dec.send_data(packet.data, None, None, None) {
            Err(e) if e.is_again() => {
                // If the decoder did not consume all data, output all
                // pending pictures and send pending data to the decoder
                // until it is all used up.
                loop {
                    handle_pending_pictures(&mut dec, false);

                    match dec.send_pending_data() {
                        Err(e) if e.is_again() => continue,
                        Err(e) => {
                            panic!("Error sending pending data to the decoder: {}", e);
                        }
                        _ => break,
                    }
                }
            }
            Err(e) => {
                panic!("Error sending data to the decoder: {}", e);
            }
            _ => (),
        }

        // Handle all pending pictures before sending the next data.
        handle_pending_pictures(&mut dec, false);
    }

    // Handle all pending pictures that were not output yet.
    handle_pending_pictures(&mut dec, true);

    Ok(())
}
