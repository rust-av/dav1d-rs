# libdav1d bindings [![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Actions Status](https://github.com/rust-av/dav1d-rs/workflows/dav1d/badge.svg)](https://github.com/rust-av/dav1d-rs/actions)

It is a simple [binding][1] and safe abstraction over [dav1d][2].

## Building

By default the bindings are generated using the headers and libraries that ought to be present in the system. However a cargo feature also allows to optionally build and statically link libdav1d into the -sys bindings:

```shell
$ cargo build --features=build
```

## TODO
- [x] Simple bindings
- [x] Safe abstraction
- [ ] Examples

[1]: https://github.com/rust-lang-nursery/rust-bindgen
[2]: https://code.videolan.org/videolan/dav1d
