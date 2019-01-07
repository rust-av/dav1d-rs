#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/dav1d.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    #[test]
    fn version() {
        println!("{}", unsafe {
            CStr::from_ptr(dav1d_version()).to_string_lossy()
        });
    }
}
