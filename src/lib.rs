pub use dav1d_sys::*;

use std::ffi::c_void;
use std::i64;
use std::mem;
use std::ptr;
use std::sync::Arc;

#[derive(Debug)]
pub struct Decoder {
    dec: *mut Dav1dContext,
}

impl Decoder {
    pub fn new() -> Self {
        unsafe {
            let mut settings = mem::uninitialized();
            let mut dec = mem::uninitialized();

            dav1d_default_settings(&mut settings);

            let ret = dav1d_open(&mut dec, &settings);

            if ret != 0 {
                panic!("Cannot instantiate the default decoder {}", ret);
            }

            Decoder { dec }
        }
    }

    pub fn flush(&self) {
        unsafe {
            dav1d_flush(self.dec);
        }
    }

    pub fn send_data<T: AsRef<[u8]>>(&mut self, buf: T) -> Result<(), i32> {
        let buf = buf.as_ref();
        let len = buf.len();
        unsafe {
            let mut data: Dav1dData = mem::zeroed();
            let ptr = dav1d_data_create(&mut data, len);
            ptr::copy_nonoverlapping(buf.as_ptr(), ptr, len);
            let ret = dav1d_send_data(self.dec, &mut data);
            if ret < 0 {
                Err(i32::from(ret))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_picture(&mut self) -> Result<Picture, i32> {
        unsafe {
            let mut pic: Dav1dPicture = mem::zeroed();
            let ret = dav1d_get_picture(self.dec, &mut pic);

            if ret < 0 {
                Err(i32::from(ret))
            } else {
                Ok(Picture { pic: Arc::new(pic) })
            }
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { dav1d_close(&mut self.dec) };
    }
}

unsafe impl Send for Decoder {}

#[derive(Debug)]
pub struct Picture {
    pic: Arc<Dav1dPicture>,
}

pub enum PixelLayout {
    I400,
    I420,
    I422,
    I444,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
pub struct BitsPerComponent(pub usize);

impl Picture {
    pub fn stride(&self, component: usize) -> i32 {
        (*self.pic).stride[component] as i32
    }

    pub fn plane_data(&self, component: usize) -> *mut c_void {
        (*self.pic).data[component]
    }

    pub fn bit_depth(&self) -> usize {
        (*self.pic).p.bpc as usize
    }

    pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
        unsafe {
            match (*(*self.pic).seq_hdr).hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
            }
        }
    }

    pub fn width(&self) -> u32 {
        (*self.pic).p.w as u32
    }

    pub fn height(&self) -> u32 {
        (*self.pic).p.h as u32
    }

    pub fn pixel_layout(&self) -> PixelLayout {
        match (*self.pic).p.layout {
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
            _ => PixelLayout::Unknown,
        }
    }

    pub fn timestamp(&self) -> Option<i64> {
        let ts = (*self.pic).m.timestamp;
        if ts == i64::MIN {
            None
        } else {
            Some(ts)
        }
    }

    pub fn duration(&self) -> i64 {
        (*self.pic).m.duration as i64
    }
}

impl Drop for Picture {
    fn drop(&mut self) {
        unsafe {
            dav1d_picture_unref(Arc::get_mut(&mut self.pic).unwrap());
        }
    }
}
