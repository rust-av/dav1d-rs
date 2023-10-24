use std::env;
use std::path::PathBuf;

mod build {
    use super::*;
    use std::path::Path;
    use std::process::{Command, Stdio};

    const REPO: &str = "https://code.videolan.org/videolan/dav1d.git";
    const TAG: &str = "1.2.1";

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
            runner!("git", "clone", "--depth", "1", "-b", TAG, REPO, &source);
        } else {
            runner!(
                "git",
                "-C",
                source.to_str().unwrap(),
                "fetch",
                "--depth",
                "1",
                "origin",
                TAG
            );
            runner!(
                "git",
                "-C",
                source.to_str().unwrap(),
                "checkout",
                "FETCH_HEAD"
            );
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
    system_deps::Config::new()
        .add_build_internal("dav1d", build::build_from_src)
        .probe()
        .unwrap();

    pkg_config::Config::new()
        .range_version("1.0.0"..="1.2.1")
        .probe("dav1d")
        .unwrap();
}
