# dav1d-tools

A command line AV1 decoder built on the safe [`dav1d`] bindings.

It mirrors the upstream `dav1d` command line tool:

- demuxers for IVF, Annex B and Section 5 streams, with content detection;
- muxers for `yuv4mpeg2`, raw `yuv`, `md5` and `null`;
- the same options for limits, skipping, threading, film grain, operating
  points, in-loop filters, checksum verification and per-frame output.

```sh
cargo install dav1d-tools
dav1d-tools -i input.ivf -o output.y4m
dav1d-tools -i input.ivf --verify 0123456789abcdef0123456789abcdef
```

See `dav1d-tools --help` for the full option list. `--realtime`,
`--realtimecache`, `--cpumask`, `--negstride` and the `xxh3` muxer are not
implemented.

[`dav1d`]: https://crates.io/crates/dav1d
