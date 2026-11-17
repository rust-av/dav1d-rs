# AGENTS.md

Guidance for automated coding agents working in this repository.

## What this repo is

Rust bindings for [dav1d](https://code.videolan.org/videolan/dav1d) (AV1 decoder), organized as a Cargo workspace:

- `dav1d` (root, `src/lib.rs`) — safe wrapper API.
- `dav1d-sys` (`dav1d-sys/`) — hand-written FFI (`dav1d-sys/src/lib.rs`) + `build.rs` using `system-deps` (finds `libdav1d` or builds it internally).
- `dav1d-tools` (`tools/`) — separate crate with a work-alike of the `dav1d` CLI built on the safe wrapper; versioned and published independently.

## Key commands

```sh
cargo fmt --all
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features

# Force a vendored build of dav1d 1.5.4 (matches CI / avoids system lib drift)
SYSTEM_DEPS_DAV1D_BUILD_INTERNAL=always cargo test --workspace --all-features
```

CI runs on Linux and Windows (`.github/workflows/dav1d.yml`). The published wrapper crates have an MSRV of **1.85** (Rust 2021), checked by the `msrv` job with `cargo check -p dav1d -p dav1d-sys --all-features --all-targets`. `dav1d-tools` is a separate crate with its own MSRV of **1.88** (through `winnow-args`) and is not part of that check.

## Conventions and pitfalls

- **`dav1d-sys` is ABI-critical.** Bindings are hand-written (no bindgen). Keep struct layout, field types/order, enums/constants, and `#[repr(...)]` identical to dav1d's public headers for the supported version range. When touching layout-sensitive types, update the size assertions in `dav1d-sys/src/lib.rs` (`tests::size`).
- **Dependencies are centralized** in the root `[workspace.dependencies]`; member crates inherit them with `workspace = true`. Update versions there, not in each crate.
- **Safe API lives in `src/lib.rs`.** Preserve existing behavior and soundness: `Send`/`Sync` impls, picture allocator safety contracts, and the `Again`/pending-data flow in `Decoder`.
- **`dav1d` re-exports `dav1d_sys::Dav1dPicture` and exposes `Picture::dav1d_picture()`** for interop with native consumers (e.g. libplacebo). Keep that escape hatch working when evolving the wrapper.
- **Monochrome (I400) has no chroma planes.** `Plane::as_ref()`/`plane_data_geometry` must not fabricate a slice for U/V on grayscale frames.
- **Don't bump crate versions or tags** unless the task explicitly calls for a release. A `dav1d`/`dav1d-sys` release bumps `Cargo.toml`, `dav1d-sys/Cargo.toml`, and the root `dav1d-sys` dependency together. `dav1d-tools` is versioned and released separately.
- Keep changes minimal and consistent with surrounding style; avoid drive-by refactors. Run fmt/clippy/tests before finishing.
