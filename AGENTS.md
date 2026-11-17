# AGENTS.md

Guidance for automated coding agents working in this repository.

## What this repo is

Rust bindings for [dav1d](https://code.videolan.org/videolan/dav1d) (AV1 decoder), organized as a Cargo workspace:

- `dav1d` (root, `src/lib.rs`) — safe wrapper API.
- `dav1d-sys` (`dav1d-sys/`) — hand-written FFI (`dav1d-sys/src/lib.rs`) + `build.rs` using `system-deps` (finds `libdav1d` or builds it internally).
- `dav1d-tools` (`tools/`) — internal CLI/example binary, not published.

## Key commands

```sh
cargo fmt --all
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features

# Force a vendored build of dav1d 1.5.4 (matches CI / avoids system lib drift)
SYSTEM_DEPS_DAV1D_BUILD_INTERNAL=always cargo test --workspace --all-features
```

CI runs on Linux and Windows (`.github/workflows/dav1d.yml`). MSRV is **1.85** (Rust 2021) and is checked by the `msrv` job with `cargo check --workspace --all-features --all-targets`.

## Conventions and pitfalls

- **`dav1d-sys` is ABI-critical.** Bindings are hand-written (no bindgen). Keep struct layout, field types/order, enums/constants, and `#[repr(...)]` identical to dav1d's public headers for the supported version range. When touching layout-sensitive types, update the size assertions in `dav1d-sys/src/lib.rs` (`tests::size`).
- **Safe API lives in `src/lib.rs`.** Preserve existing behavior and soundness: `Send`/`Sync` impls, picture allocator safety contracts, and the `Again`/pending-data flow in `Decoder`.
- **`dav1d` re-exports `dav1d_sys::Dav1dPicture` and exposes `Picture::dav1d_picture()`** for interop with native consumers (e.g. libplacebo). Keep that escape hatch working when evolving the wrapper.
- **Monochrome (I400) has no chroma planes.** `Plane::as_ref()`/`plane_data_geometry` must not fabricate a slice for U/V on grayscale frames.
- **Don't bump crate versions or tags** unless the task explicitly calls for a release. Release commits historically bump `Cargo.toml`, `dav1d-sys/Cargo.toml`, `tools/Cargo.toml`, and the root `dav1d-sys` dependency together.
- Keep changes minimal and consistent with surrounding style; avoid drive-by refactors. Run fmt/clippy/tests before finishing.
