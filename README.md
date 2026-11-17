# dav1d-rs

[![crates.io](https://img.shields.io/crates/v/dav1d.svg)](https://crates.io/crates/dav1d)
[![docs.rs](https://docs.rs/dav1d/badge.svg)](https://docs.rs/dav1d)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/rust-av/dav1d-rs/actions/workflows/dav1d.yml/badge.svg)](https://github.com/rust-av/dav1d-rs/actions/workflows/dav1d.yml)

Rust bindings for [dav1d][dav1d], the fast and small AV1 decoder.

This repository provides two crates:

- **`dav1d`** — a safe, idiomatic wrapper around the decoder.
- **`dav1d-sys`** — low-level FFI bindings to `libdav1d` (found via `pkg-config`/`system-deps`, or built from source).

## Usage

Add the wrapper crate to your project:

```toml
[dependencies]
dav1d = "0.12"
```

A minimal decoding loop looks like this:

```rust,no_run
use dav1d::{Decoder, Error, Settings};

fn decode() -> Result<(), Error> {
    let mut decoder = Decoder::with_settings(&Settings::new())?;
    // Feed AV1 OBUs (for example, packets demuxed from IVF/MP4/WebM).
    decoder.send_data(Vec::<u8>::new(), None, None, None)?;

    loop {
        match decoder.get_picture() {
            Ok(picture) => {
                let y = picture.plane(dav1d::PlanarImageComponent::Y);
                let _ = (picture.width(), picture.height(), y.as_ref());
            }
            Err(Error::Again) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
```

See the crate documentation on [docs.rs](https://docs.rs/dav1d) for the full API, including picture allocation hooks, metadata accessors, and the `dav1d_picture()` escape hatch for interoperating with libraries that consume dav1d's native structures (e.g. libplacebo).

## Command line tool

The workspace also contains `dav1d-tools`, a separate crate with a work-alike of the `dav1d` command line tool. It detects IVF, Annex B and Section 5 streams, writes y4m, raw yuv or md5 output, and supports the same options (`--limit`, `--skip`, `--verify`, per-frame output and so on):

```sh
cargo run -p dav1d-tools -- -i input.ivf -o output.y4m
```

It is versioned and published independently of the wrapper crates, and requires Rust 1.88 because of its `winnow-args` dependency.

## Building

You need a C toolchain, `meson`, and `ninja` if `libdav1d` is not already installed system-wide. `system-deps` locates the library through `pkg-config`; when no suitable system copy is found it can build dav1d internally.

Useful environment variables (see [`system-deps`](https://docs.rs/system-deps)):

- `PKG_CONFIG_PATH` — additional `pkg-config` search paths.
- `SYSTEM_DEPS_DAV1D_SEARCH_NATIVE` / `SYSTEM_DEPS_DAV1D_LIB` — override discovery/linking.
- `SYSTEM_DEPS_DAV1D_BUILD_INTERNAL=always` — force a vendored build of dav1d.

Run the test suite with:

```sh
cargo test --workspace --all-features
```

### Windows (vcpkg, x64)

```powershell
choco install pkgconfiglite
vcpkg install dav1d:x64-windows
# add %VCPKG_INSTALLATION_ROOT%\installed\x64-windows\lib\pkgconfig to PKG_CONFIG_PATH
cargo build --workspace
```

(Use `x86-windows` for a 32-bit build.)

## Supported dav1d versions

The bindings target `libdav1d` **>= 1.3.0 and <= 1.5.4**. Older releases are unsupported; newer dav1d releases may work but are not part of the tested matrix yet.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development workflow, testing, and release notes, and [AGENTS.md](AGENTS.md) for guidance aimed at automated coding agents.

## License

MIT — see [LICENSE](LICENSE).

[dav1d]: https://code.videolan.org/videolan/dav1d
