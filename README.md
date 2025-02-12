<meta charset="utf-8"/>

# niffler

Simple and transparent support for compressed files.

This library provides two main features:
- sniffs out compression formats from input files and returns a
  Read trait object ready for consumption.
- Create a Writer initialized with compression ready for writing.

The goal is to lower the barrier to open and use a file, especially in
bioinformatics workflows.

[![build-status]][github-actions]
[![Crates.io](https://img.shields.io/crates/v/niffler.svg)](https://crates.io/crates/niffler)
[![Documentation](https://docs.rs/niffler/badge.svg)](https://docs.rs/niffler/)

[build-status]: https://github.com/luizirber/niffler/workflows/CI/badge.svg
[github-actions]: https://github.com/luizirber/niffler/actions?query=workflow%3ACI


# Example

```rust
use niffler::{Error, compression};
# fn main() -> Result<(), Error> {
# #[cfg(feature = "gz")] {
let mut buffer = Vec::new();

{
  let mut writer = niffler::get_writer(Box::new(&mut buffer), compression::Format::Gzip, niffler::Level::Nine)?;
  writer.write_all(b"hello")?;
}

# assert_eq!(&buffer, &[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255, 203, 72, 205, 201, 201, 7, 0, 134, 166, 16, 54, 5, 0, 0, 0]);

let (mut reader, compression) = niffler::get_reader(Box::new(&buffer[..]))?;

let mut contents = String::new();
reader.read_to_string(&mut contents)?;

assert_eq!(compression, niffler::compression::Format::Gzip);
assert_eq!(contents, "hello");
# }
# Ok(())
# }
```

## Selecting compression formats

By default all supported compression formats are enabled,
using their default features or with an optimized subset of features.

The crates used for decompression provide a number of features that can have
a significant impact on performance.
For advanced uses,
like selecting specific features for any of the compression crates,
or if you are working on systems that do not support some of the compression formats,
you can disable default features and select the compression formats you want in niffler,
and then you must select the appropriate features for the specific compression crate
implementing that format.
For example,
you can use this in your `Cargo.toml` to select only the `gz` support,
and choose your preferred gzip implementation:
```toml
niffler = { version = "3.0.0", default-features = false, features = ["gz"] }
flate2 = { version = "1.0.35", default-features = false, features = ["zlib-ng"] }
```
These are the niffler features, and the compression crate used.
Check [Cargo.toml](Cargo.toml) for specific versions when adding to your project.
| niffler feature | Crate | Crate features |
| --- | --- | --- |
| `bgz` | [bgzip](https://lib.rs/crates/bgzip) | [Check on docs.rs](https://docs.rs/crate/bgzip/latest/features) |
| `bz2` | [bzip2](https://lib.rs/crates/bzip2) | [Check on docs.rs](https://docs.rs/crate/bzip2/latest/features) |
| `gz` | [flate2](https://lib.rs/crates/flate2) | [Check on docs.rs](https://docs.rs/crate/flate2/latest/features) |
| `lzma` | [liblzma](https://lib.rs/crates/liblzma) | [Check on docs.rs](https://docs.rs/crate/liblzma/latest/features) |
| `zstd` | [zstd](https://lib.rs/crates/zstd) | [Check on docs.rs](https://docs.rs/crate/zstd/latest/features) |

You can also run `cargo tree` to verify what features are enabled by default,
and better guide you when choosing the features you want.
```bash
❯ cargo tree -f '{p} {f}' -e no-dev --depth 1
niffler v3.0.0 bgz,bz2,default,gz,lzma,zstd
├── bgzip v0.3.1 default,flate2,log,rayon,rust_backend
├── bzip2 v0.5.0 default
├── cfg-if v1.0.0
├── flate2 v1.0.35 any_impl,default,miniz_oxide,rust_backend
├── liblzma v0.3.5 bindgen,default
├── thiserror v2.0.11 default,std
└── zstd v0.13.2 arrays,default,legacy,zdict_builder
```

This level of control is especially useful if you need to link with an external C/C++ project that
has specific requirements,
or if you want to harmonize features with other crates you have in your projects.

You can still use `niffler::sniff()` to find what is the compression format,
even if any feature is disabled.
But if you try to use `niffler::get_reader` or `niffler::get_writer` for a feature that was not enabled,
it will throw a runtime error.

## Minimum supported Rust version

Currently the minimum supported Rust version is 1.74.0.

## Similar project

Many similar projects exist in other languages:

- C: [rbread](https://github.com/ocxtal/rbread)
- Python: [xphyle](https://github.com/jdidion/xphyle), [xopen](https://github.com/marcelm/xopen)
- Perl: [AnyUncompress](https://perldoc.perl.org/IO/Uncompress/AnyUncompress.html)
- go: [Archiver](https://github.com/mholt/archiver)

## Development

niffler development is open, and [pull requests](https://github.com/luizirber/niffler/pulls) are welcome!

Before creating your pull request, please try to write a test and benchmark (if possible).
Some commands we suggest running to help with these tasks:
```bash
cargo fmt
cargo test
cargo clippy
```

To run tests use:
```bash
cargo test --all-features
```

To test benchmark run:
```bash
cargo test --benches --all-features
```

To run all benchmark use:
```bash
cargo bench --all-features
```

As a shortcut, you can also run `make`,
which will execute all these commands.

## License

Licensed under either of these:

 * Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0> )
 * MIT License ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT> )

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work, as defined
in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
