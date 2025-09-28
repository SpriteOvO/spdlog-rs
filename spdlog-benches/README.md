# spdlog-benches

## Benchmarks

[The tracking page] for benchmark changes for each commit during development.

Run `cargo +nightly bench --features multi-thread` in the root directory of this repository for benchmarking.

The following results are generated on `Windows 10 64 bit` and `Intel i9-10900KF CPU @ 3.70GHz` with `cargo 1.92.0-nightly (f2932725b 2025-09-24)`.

### `spdlog-rs` (0.5.0)

- Default features

  ```
  test bench_1_file               ... bench:         201.82 ns/iter (+/- 10.63)
  test bench_2_file_async         ... bench:         244.50 ns/iter (+/- 11.37)
  test bench_3_rotating_file_size ... bench:         197.51 ns/iter (+/- 36.44)
  test bench_4_rotating_daily     ... bench:         200.50 ns/iter (+/- 9.47)
  test bench_5_level_off          ... bench:           1.37 ns/iter (+/- 0.13)
  ```

- Enable `flexible-string` feature

  ```
  test bench_1_file               ... bench:         170.79 ns/iter (+/- 10.30)
  test bench_2_file_async         ... bench:         247.40 ns/iter (+/- 15.25)
  test bench_3_rotating_file_size ... bench:         172.91 ns/iter (+/- 27.18)
  test bench_4_rotating_daily     ... bench:         173.12 ns/iter (+/- 10.84)
  test bench_5_level_off          ... bench:           1.43 ns/iter (+/- 0.02)
  ```

<details><summary><b>Compared with other Rust crates</b></summary>

#### Disclaimer

I'm not entirely familiar with using the other Rust crates below, so if you find a bug or something worth improving in the benchmark code, feel free to open an issue to let me know.

### `tracing` (0.1.41), `tracing-subscriber` (0.3.20), `tracing-appender` (0.2.3)

```
test bench_1_file               ... bench:       2,479.67 ns/iter (+/- 224.87)
test bench_2_file_async         ... bench:         699.95 ns/iter (+/- 47.68)
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ... bench:       2,473.01 ns/iter (+/- 244.29)
test bench_5_level_off          ... bench:           0.39 ns/iter (+/- 0.03)
```

### `slog` (2.7.0), `sloggers` (2.2.0)

```
test bench_1_file                     ...                   unavailable
test bench_2_file_async               ... bench:         446.12 ns/iter (+/- 28.20)
test bench_3_rotating_file_size_async ... bench:         447.39 ns/iter (+/- 30.20)
test bench_4_rotating_daily           ...                   unavailable
test bench_5_level_off                ... bench:           1.76 ns/iter (+/- 0.10)
```

### `flexi_logger` (0.31.4)

```
test bench_1_file               ... bench:       1,172.30 ns/iter (+/- 73.46)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ... bench:       1,206.46 ns/iter (+/- 109.38)
test bench_4_rotating_daily     ... bench:       1,510.36 ns/iter (+/- 101.03)
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.02)
```

### `log4rs` (1.4.0)

```
test bench_1_file               ... bench:       2,931.94 ns/iter (+/- 201.13)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ... bench:       2,968.21 ns/iter (+/- 141.19)
test bench_4_rotating_daily     ...                   unavailable
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.01)
```

### `fern` (0.7.1)

```
test bench_1_file               ... bench:       2,927.74 ns/iter (+/- 180.88)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ...                   unavailable
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.02)
```

### `ftlog` (0.2.15)

```
test bench_1_file               ...                   unavailable
test bench_2_file_async         ... bench:         233.98 ns/iter (+/- 22.20)
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ... bench:         239.08 ns/iter (+/- 20.10)
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.01)
```

### `fast_log` (1.7.7)

```
test bench_1_file                     ...                   unavailable
test bench_2_file_async               ... bench:         246.59 ns/iter (+/- 3,179.82)
test bench_3_rotating_file_size_async ... bench:         255.32 ns/iter (+/- 631.78)
test bench_4_rotating_daily_async     ... bench:         206.69 ns/iter (+/- 653.65)
test bench_5_level_off                ... bench:           0.20 ns/iter (+/- 0.02)
```

### `logforth` (0.27.0)

```
test bench_1_file                     ...                   unavailable
test bench_2_file_async               ... bench:         780.12 ns/iter (+/- 44.93)
test bench_3_rotating_file_size_async ... bench:         918.14 ns/iter (+/- 47.00)
test bench_4_rotating_daily_async     ... bench:         907.25 ns/iter (+/- 71.37)
test bench_5_level_off                ... bench:           5.88 ns/iter (+/- 0.43)
```
</details>

<details><summary><b>Compared with C++ spdlog</b></summary>

### `spdlog-rs` (0.5.0)

- Default features (corresponds to C++ `spdlog` using standard `<format>`)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.08 secs          3092991/sec
    [info] rotating_mt                    Elapsed: 0.08 secs          3096838/sec
    [info] daily_mt                       Elapsed: 0.08 secs          3181888/sec
    [info] level-off                      Elapsed: 0.00 secs        567150635/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3605209/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3519391/sec
    [info] daily_mt                       Elapsed: 0.08 secs          3332786/sec
    [info] level-off                      Elapsed: 0.00 secs        687947165/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 136 = 1088 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.4880718 secs   2048878/sec
    [info] Elapsed: 0.4717359 secs   2119830/sec
    [info] Elapsed: 0.4589705 secs   2178789/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.1043783 secs   9580535/sec
    [info] Elapsed: 0.1039465 secs   9620333/sec
    [info] Elapsed: 0.104625 secs    9557945/sec
    ```

- Enable `flexible-string` feature (corresponds to C++ `spdlog` using `fmt` library)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3660086/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3666522/sec
    [info] daily_mt                       Elapsed: 0.07 secs          3597096/sec
    [info] level-off                      Elapsed: 0.00 secs        508957654/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3572576/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3410138/sec
    [info] daily_mt                       Elapsed: 0.07 secs          3552902/sec
    [info] level-off                      Elapsed: 0.00 secs        724427702/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 136 = 1088 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.4305708 secs   2322498/sec
    [info] Elapsed: 0.4584145 secs   2181431/sec
    [info] Elapsed: 0.4509432 secs   2217574/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.1058637 secs   9446108/sec
    [info] Elapsed: 0.1072791 secs   9321480/sec
    [info] Elapsed: 0.1063269 secs   9404957/sec
    ```

### C++ `spdlog` (1.15.3)

Compiler `MSVC 19.44.35217.0`.

- Using standard `<format>`
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF -DSPDLOG_USE_STD_FORMAT=ON`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.12 secs        2,118,970/sec
    [info] rotating_mt                    Elapsed: 0.13 secs        1,890,720/sec
    [info] daily_mt                       Elapsed: 0.12 secs        2,089,322/sec
    [info] level-off                      Elapsed: 0.00 secs      139,977,603/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.18 secs        1,424,666/sec
    [info] rotating_mt                    Elapsed: 0.19 secs        1,329,264/sec
    [info] daily_mt                       Elapsed: 0.18 secs        1,364,071/sec
    [info] level-off                      Elapsed: 0.00 secs      139,899,272/sec
    ```

  - Async

    ```
    [info] -------------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 152 = 1216 KB
    [info] Total iters  : 3
    [info] -------------------------------------------------
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: block
    [info] *********************************
    [info] Elapsed: 2.132892 secs    468846/sec
    [info] Elapsed: 2.1042392 secs   475231/sec
    [info] Elapsed: 2.1130006 secs   473260/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 0.4682991 secs   2135387/sec
    [info] Elapsed: 0.4668421 secs   2142051/sec
    [info] Elapsed: 0.4589271 secs   2178995/sec
    ```

- Using `fmt` library
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.06 secs        3,945,408/sec
    [info] rotating_mt                    Elapsed: 0.06 secs        4,016,483/sec
    [info] daily_mt                       Elapsed: 0.06 secs        3,872,792/sec
    [info] level-off                      Elapsed: 0.00 secs      136,061,826/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.10 secs        2,497,243/sec
    [info] rotating_mt                    Elapsed: 0.10 secs        2,421,563/sec
    [info] daily_mt                       Elapsed: 0.10 secs        2,390,189/sec
    [info] level-off                      Elapsed: 0.00 secs      136,054,421/sec
    ```

  - Async

    ```
    [info] -------------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 408 = 3264 KB
    [info] Total iters  : 3
    [info] -------------------------------------------------
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: block
    [info] *********************************
    [info] Elapsed: 1.7362919 secs   575940/sec
    [info] Elapsed: 1.7509939 secs   571104/sec
    [info] Elapsed: 1.7574208 secs   569015/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 0.3349014 secs   2985953/sec
    [info] Elapsed: 0.3340692 secs   2993391/sec
    [info] Elapsed: 0.3259194 secs   3068243/sec
    ```

</details>

[The tracking page]: https://spriteovo.github.io/spdlog-rs/dev/benchmarks/
