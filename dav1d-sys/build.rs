use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod build {
    use super::*;
    use std::path::Path;
    use std::process::{Command, Stdio};

    const REPO: &str = "https://code.videolan.org/videolan/dav1d.git";

    macro_rules! runner {
        ($cmd:expr, $($arg:expr),*) => {
            Command::new($cmd)
                $(.arg($arg))*
                .stderr(Stdio::inherit())
                .stdout(Stdio::inherit())
                .output()
                .expect(concat!($cmd, " failed"));

        };
    }

    pub fn build_from_src(
        lib: &str,
        version: &str,
    ) -> Result<system_deps::Library, system_deps::BuildInternalClosureError> {
        let build_dir = "build";
        let release_dir = "release";

        let source = PathBuf::from(env::var("OUT_DIR").unwrap()).join("dav1d");
        let build_path = source.join(build_dir);
        let release_path = source.join(release_dir);

        if !Path::new(&source.join(".git")).exists() {
            runner!("git", "clone", "--depth", "1", REPO, &source);
        } else {
            runner!("git", "-C", source.to_str().unwrap(), "pull");
        }

        runner!(
            "meson",
            "setup",
            "-Ddefault_library=static",
            "--prefix",
            release_path.to_str().unwrap(),
            build_path.to_str().unwrap(),
            source.to_str().unwrap()
        );
        runner!("ninja", "-C", build_path.to_str().unwrap());
        runner!("meson", "install", "-C", build_path.to_str().unwrap());

        let pkg_dir = build_path.join("meson-private");
        system_deps::Library::from_internal_pkg_config(&pkg_dir, lib, version)
    }
}

fn main() {
    let libs = system_deps::Config::new()
        .add_build_internal("dav1d", |lib, version| build::build_from_src(lib, version))
        .probe()
        .unwrap();

    let libs = libs.get("dav1d").unwrap();

    let headers = libs.include_paths.clone();

    let mut builder = bindgen::builder()
        .blacklist_type("max_align_t")
        .size_t_is_usize(true)
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
