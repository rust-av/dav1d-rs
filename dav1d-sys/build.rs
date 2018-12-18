// build.rs

extern crate bindgen;
#[cfg(unix)]
extern crate pkg_config;

use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let libs = pkg_config::Config::new().probe("dav1d").unwrap();

    use std::io::Write;

    let headers = libs.include_paths.clone();

    let mut builder = bindgen::builder()
        .blacklist_type("max_align_t")
        .rustfmt_bindings(false)
        .header("data/dav1d.h");

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    // Manually fix the comment so rustdoc won't try to pick them
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_path.join("dav1d.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
