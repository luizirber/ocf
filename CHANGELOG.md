# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [3.0.0] - 2025-02-12

### Changed

- Clean up feature usage. Removed many transitive features,
  and `xz` feature (use `lzma` feature instead).
  For projects that disable default features see [updated instructions]
  on how to use specific compression formats and their crate specific
  feature flags (#68)

[updated instructions]: https://docs.rs/niffler/3.0.0/niffler/index.html#selecting-compression-formats

### Added

- Explicit `wasm` feature for compression formats supported in webassembly
  (`wasm32-unknown-unknown` target) (#72)
- Pixi configuration for local development (#71)
- CI: run benchmarks and upload results to codspeed.io
- Deps: bump `bzip2` to `0.5.1`.

### Fixed

## [2.7.0] - 2025-01-24

### Added

- allow gz_lib-ng (without compat) (#69)
- CI: use cargo-semver-checks to verify breaking changes (#66)

### Changed

- Bump MRSV to 1.74

## [2.6.0] - 2024-06-08

### Changed

- Replace xz2 to liblzma
- Bump MRSV to 1.65
- Support for multistream bzip2 files by default (#62)

### Fixed

- Update bzip to 0.4.4
- Update bgzip to 0.3
- Update zstd 0.13

## [2.5.0] - 2023-01-12

### Added

- Add support of zstd in send (#59)
- Add continuous deployment workflow (#57)
- Add features to allow user to select features of backend (#54)

### Changed

- Bump MSRV to 1.57.0 (#59)
- Bump MSRV to 1.56.1 (#54)

### Fixed

- Deduplicate implementation of get_first_{bytes|five} functions (#52)

## [2.4.0] - 2021-12-25

### Added

- Zstd basic support (#44)
- GitHub issues templates (#47)

### Changed

- Bump MSRV to 1.51 (#48)

## [2.3.2] - 2021-05-27

### Changed

- Bump bgzip version to 0.2.0, removing dependency on failure

## [2.3.1] - 2020-12-08

### Fixed

- Maintain backward compatibility by re-exporting compression::Level

## [2.3.0] - 2020-12-08

### Added

- Support Seek and Send and prepare for future extension (#37)

## [2.2.0] - 2020-07-01

### Changed

- Make gz compression optional too, matching behavior of other compression formats (#29)

## [2.1.1] - 2020-06-30

### Changed

- Replace `GzDecoder` with `MultiGzDecoder` for gzip decompression (#28)

## [2.1.0] - 2020-06-06

### Added

- `sniff` function to detect compression format, and add benchmarks for most methods (#25)
- More test on compression level conversion (#26)

## [2.0.1] - 2020-05-20

### Added

- Clone and Copy traits to pub enums [@schmidmt](https://github.com/schmidmt).

## [2.0.0] - 2020-04-27

### Added

- New functions: `get_reader`, `get_writer`, `from_path`, `to_path`,
- Compression format and compression level enums
- Documentation with examples
- CI using GitHub Actions

### Changed

- Replace failure with thiserror for error handling

### Fixed

- Reorganize crate internal organization

### Removed

- All previous functions and enums were renamed and reorganized.

## [1.0.0] - 2019-12-07

### Added

- Rename crate from `ocf` to `niffler`
- Import codebase from sourmash repo (which copied it from the yacrd repo)

[unreleased]: https://github.com/luizirber/niffler/compare/v3.0.0...HEAD
[3.0.0]: https://github.com/luizirber/niffler/compare/v2.7.0..v3.0.0
[2.7.0]: https://github.com/luizirber/niffler/compare/v2.6.0..v2.7.0
[2.6.0]: https://github.com/luizirber/niffler/compare/v2.5.0..v2.6.0
[2.5.0]: https://github.com/luizirber/niffler/compare/v2.4.0..v2.5.0
[2.4.0]: https://github.com/luizirber/niffler/compare/v2.3.2..v2.4.0
[2.3.2]: https://github.com/luizirber/niffler/compare/v2.3.1..v2.3.2
[2.3.1]: https://github.com/luizirber/niffler/compare/v2.3.0..v2.3.1
[2.3.0]: https://github.com/luizirber/niffler/compare/v2.2.0..v2.3.0
[2.2.0]: https://github.com/luizirber/niffler/compare/v2.0.1..v2.2.0
[2.0.1]: https://github.com/luizirber/niffler/compare/v2.0.0..v2.0.1
[2.0.0]: https://github.com/luizirber/niffler/compare/v1.0.0..v2.0.0
[1.0.0]: https://github.com/luizirber/niffler/releases/tag/v1.0.0
