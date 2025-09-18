# spdlog-rs

[![](https://img.shields.io/badge/github-spdlog--rs-blue?style=flat-square&logo=github)](https://github.com/SpriteOvO/spdlog-rs)
[![](https://img.shields.io/crates/v/spdlog-rs?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://crates.io/crates/spdlog-rs)
[![](https://img.shields.io/badge/docs.rs-spdlog--rs-ff69b4?style=flat-square&logo=rust)](https://docs.rs/spdlog-rs)
[![](https://img.shields.io/github/actions/workflow/status/SpriteOvO/spdlog-rs/ci.yml?branch=main&style=flat-square&logo=githubactions&logoColor=white)](https://github.com/SpriteOvO/spdlog-rs/actions/workflows/ci.yml)
 
Fast, highly configurable Rust logging crate, inspired by the C++ logging library [spdlog].

## Features

 - Very fast (see [Benchmarks]).
 - Asynchronous support.
 - Compatible with `log` crate.
 - Custom log formats:
   - compile-time zero-cost pattern or runtime pattern;
   - manually implementing for more flexibility.
 - Various combinable sinks:
    - standard streams with optional color support;
    - files (single file, rotating hourly, daily, periodically or by file size);
    - platform-specific (e.g. `journald` for Linux and `OutputDebugStringW` for Windows);
    - ... and able to implement one yourself.
 - Structured logging.
 - Configuring via environment variables.
 - Readable level filters.

## Getting started

Add this to `Cargo.toml`:
```toml
[dependencies]
spdlog-rs = "0.4"
```

The documentation of this crate is hosted on [docs.rs], and you can learn examples under [./examples] directory along with it.

If you have any trouble while using this crate, please don't hesitate to [open a discussion] for help. For feature requests or bug reports, please [open an issue].

## Developments

Unreleased commits are active on [`main-dev`] branch, and [`main`] branch is only synchronized when released.

If you are going to contribute `spdlog-rs`, please make sure to check out the [`main-dev`] branch and select the [`main-dev`] branch as the base when opening PR.

## Supported Rust versions

<!--
When updating this, also update:
- .github/workflows/ci.yml
- src/lib.rs
- Cargo.toml
-->

The current minimum supported Rust version is 1.63.

`spdlog-rs` is built against the latest Rust stable release, it is not guaranteed to build on Rust versions earlier than the minimum supported version.

`spdlog-rs` follows the compiler support policy that the latest stable version and the 3 most recent minor versions before that are always supported. For example, if the current latest Rust stable version is 1.61, the minimum supported version will not be increased past 1.58. Increasing the minimum supported version is not considered a semver breaking change as long as it complies with this policy.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[spdlog]: https://github.com/gabime/spdlog
[Benchmarks]: https://github.com/SpriteOvO/spdlog-rs/blob/main/spdlog/benches/README.md
[#25]: https://github.com/SpriteOvO/spdlog-rs/issues/25
[./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
[docs.rs]: https://docs.rs/spdlog-rs/
[open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
[open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
[`main-dev`]: https://github.com/SpriteOvO/spdlog-rs/tree/main-dev
[`main`]: https://github.com/SpriteOvO/spdlog-rs/tree/main
