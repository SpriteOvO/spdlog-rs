# spdlog-rs

[![](https://img.shields.io/crates/v/spdlog-rs?style=flat-square)](https://crates.io/crates/spdlog-rs)
[![](https://img.shields.io/github/workflow/status/SpriteOvO/spdlog-rs/CI?style=flat-square)](https://github.com/SpriteOvO/spdlog-rs/actions/workflows/ci.yml)
[![](https://img.shields.io/docsrs/spdlog-rs?style=flat-square)](https://docs.rs/spdlog-rs)
[![](https://img.shields.io/crates/l/spdlog-rs?style=flat-square)](https://github.com/SpriteOvO/spdlog-rs#license)
 
A fast and combinable Rust logging crate, inspired by the C++ logging library [spdlog].

## Features

 - Very fast (see [Benchmarks](#Benchmarks) below).
 - Various log targets:
    - Standard streams with optional colors.
    - Files.
    - Rotating log files by file size.
    - Rotating log files hourly.
    - Rotating log files daily.
    - ... (more targets are implementing, PRs are welcome)
    - Extendable with custom log targets.
 - Compatible with [log crate] (optional).
 - Configured via environment variable.
 - Custom formatting.
 - Log filtering - log levels can be modified in runtime as well as in compile time.

## Getting started

Add this to `Cargo.toml`:
```toml
[dependencies]
spdlog-rs = "0.2"
```

The documentation of this crate is hosted on [docs.rs], and you can find examples under [./examples] directory.

If you have any questions or need help while using this crate, feel free to [open a discussion]. For feature requests or bug reports, please [open an issue].

## Benchmarks

Run `cargo +nightly bench` in the root directory of this repository for benchmarking.

The following results are generated with `Windows 10 64 bit` and `Intel i9-10900KF CPU @ 3.70GHz`.

Disclaimer, I'm not entirely familiar with using the other Rust crates below, so if you find a bug or something worth improving in the benchmark code, feel free to open an issue to let me know.

### `spdlog-rs` (0.1.0)

Default features
```
test bench_file               ... bench:         376 ns/iter (+/- 12)
test bench_level_off          ... bench:           1 ns/iter (+/- 0)
test bench_rotating_daily     ... bench:         380 ns/iter (+/- 12)
test bench_rotating_file_size ... bench:         379 ns/iter (+/- 50)
```

Enable `flexible-string` feature
```
test bench_file               ... bench:         166 ns/iter (+/- 10)
test bench_level_off          ... bench:           1 ns/iter (+/- 0)
test bench_rotating_daily     ... bench:         172 ns/iter (+/- 7)
test bench_rotating_file_size ... bench:         176 ns/iter (+/- 20)
```

### Compare with `slog` (2.7.0)

```
test bench_file               ... bench:         469 ns/iter (+/- 19)
test bench_level_off          ... bench:           2 ns/iter (+/- 0)
test bench_rotating_daily     ... bench:         unavailable
test bench_rotating_file_size ... bench:         480 ns/iter (+/- 13)
```

### Compare with `flexi_logger` (0.22.2)

```
test bench_file               ... bench:         673 ns/iter (+/- 11)
test bench_level_off          ... bench:           0 ns/iter (+/- 0)
test bench_rotating_daily     ... bench:         746 ns/iter (+/- 40)
test bench_rotating_file_size ... bench:         676 ns/iter (+/- 38)
```

### Compare with `log4rs` (1.0.0)

```
test bench_file               ... bench:       3,769 ns/iter (+/- 95)
test bench_level_off          ... bench:           0 ns/iter (+/- 0)
test bench_rotating_daily     ... bench:         unavailable
test bench_rotating_file_size ... bench:       3,773 ns/iter (+/- 117)
```

### Compare with `fern` (0.6.0)

```
test bench_file               ... bench:       3,687 ns/iter (+/- 101)
test bench_level_off          ... bench:         unavailable
test bench_rotating_daily     ... bench:         unavailable
test bench_rotating_file_size ... bench:         unavailable
```

### Compare with C++ `spdlog`

#### `spdlog-rs` (0.1.0)

Default features (corresponds to C++ `spdlog` using standard `<format>`)
```
[info] **********************************************************************
[info] Multi threaded: 1 threads, 250000 messages
[info] **********************************************************************
[info] basic_mt                       Elapsed: 0.13 secs          1940870/sec
[info] rotating_mt                    Elapsed: 0.13 secs          1894612/sec
[info] daily_mt                       Elapsed: 0.13 secs          1920024/sec
[info] level-off                      Elapsed: 0.00 secs        444919024/sec
[info] **********************************************************************
[info] Multi threaded: 4 threads, 250000 messages
[info] **********************************************************************
[info] basic_mt                       Elapsed: 0.14 secs          1825379/sec
[info] rotating_mt                    Elapsed: 0.14 secs          1845651/sec
[info] daily_mt                       Elapsed: 0.13 secs          1854885/sec
[info] level-off                      Elapsed: 0.00 secs        485625485/sec
```

Enable `flexible-string` feature (corresponds to C++ `spdlog` using `fmt` library)
```
[info] **********************************************************************
[info] Multi threaded: 1 threads, 250000 messages
[info] **********************************************************************
[info] basic_mt                       Elapsed: 0.08 secs          3003511/sec
[info] rotating_mt                    Elapsed: 0.08 secs          3006090/sec
[info] daily_mt                       Elapsed: 0.08 secs          3032813/sec
[info] level-off                      Elapsed: 0.00 secs        484871993/sec
[info] **********************************************************************
[info] Multi threaded: 4 threads, 250000 messages
[info] **********************************************************************
[info] basic_mt                       Elapsed: 0.06 secs          4266393/sec
[info] rotating_mt                    Elapsed: 0.06 secs          4271496/sec
[info] daily_mt                       Elapsed: 0.06 secs          4164993/sec
[info] level-off                      Elapsed: 0.00 secs        462962962/sec
```

#### C++ `spdlog` ([`4cfdc8c`])

Using standard `<format>` (compiled with `cmake -G "Visual Studio 16 2019" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF -DSPDLOG_USE_STD_FORMAT=ON`)
```
[info] **************************************************************
[info] Multi threaded: 1 threads, 250,000 messages
[info] **************************************************************
[info] basic_mt                       Elapsed: 0.15 secs        1,654,676/sec
[info] rotating_mt                    Elapsed: 0.16 secs        1,576,156/sec
[info] daily_mt                       Elapsed: 0.15 secs        1,671,424/sec
[info] level-off                      Elapsed: 0.00 secs      132,597,857/sec
[info] **************************************************************
[info] Multi threaded: 4 threads, 250,000 messages
[info] **************************************************************
[info] basic_mt                       Elapsed: 0.26 secs          965,885/sec
[info] rotating_mt                    Elapsed: 0.26 secs          964,368/sec
[info] daily_mt                       Elapsed: 0.25 secs          981,449/sec
[info] level-off                      Elapsed: 0.00 secs      135,310,673/sec
```

Using `fmt` library (compiled with `cmake -G "Visual Studio 16 2019" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF`)
```
[info] **************************************************************
[info] Multi threaded: 1 threads, 250,000 messages
[info] **************************************************************
[info] basic_mt                       Elapsed: 0.06 secs        3,917,304/sec
[info] rotating_mt                    Elapsed: 0.06 secs        3,942,073/sec
[info] daily_mt                       Elapsed: 0.07 secs        3,784,707/sec
[info] level-off                      Elapsed: 0.00 secs      148,174,490/sec
[info] **************************************************************
[info] Multi threaded: 4 threads, 250,000 messages
[info] **************************************************************
[info] basic_mt                       Elapsed: 0.11 secs        2,356,303/sec
[info] rotating_mt                    Elapsed: 0.12 secs        2,138,911/sec
[info] daily_mt                       Elapsed: 0.12 secs        2,163,183/sec
[info] level-off                      Elapsed: 0.00 secs      148,060,408/sec
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[spdlog]: https://github.com/gabime/spdlog
[log crate]: https://crates.io/crates/log
[./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
[docs.rs]: https://docs.rs/spdlog-rs/
[open a discussion]: https://github.com/SpriteOvO/spdlog-rs/discussions/new
[open an issue]: https://github.com/SpriteOvO/spdlog-rs/issues/new/choose
[`4cfdc8c`]: https://github.com/gabime/spdlog/commit/4cfdc8c5c84f696774cb9acde2f95c9e87c11a5e
