use dav1d_sys::*;

use std::mem;
use std::ptr;

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

    pub fn get_picture(&mut self) -> Result<Dav1dPicture, i32> {
        unsafe {
            let mut pic: Dav1dPicture = mem::zeroed();
            let ret = dav1d_get_picture(self.dec, &mut pic);

            if ret < 0 {
                Err(i32::from(ret))
            } else {
                Ok(pic)
            }
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { dav1d_close(&mut self.dec) };
    }
}
