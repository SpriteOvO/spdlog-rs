# Benchmarks

[The tracking page] for benchmark changes for each commit during development.

Run `cargo +nightly bench --features multi-thread` in the root directory of this repository for benchmarking.

The following results are generated on `Windows 10 64 bit` and `Intel i9-10900KF CPU @ 3.70GHz` with `cargo 1.83.0-nightly (ad074abe3 2024-10-04)`.

### `spdlog-rs` (0.4.0)

- Default features

  ```
  test bench_1_file               ... bench:         163.62 ns/iter (+/- 11.70)
  test bench_2_file_async         ... bench:         214.90 ns/iter (+/- 16.04)
  test bench_3_rotating_file_size ... bench:         184.23 ns/iter (+/- 31.93)
  test bench_4_rotating_daily     ... bench:         170.50 ns/iter (+/- 10.23)
  test bench_5_level_off          ... bench:           1.60 ns/iter (+/- 0.08)
  ```

- Enable `flexible-string` feature

  ```
  test bench_1_file               ... bench:         143.55 ns/iter (+/- 7.91)
  test bench_2_file_async         ... bench:         215.43 ns/iter (+/- 12.25)
  test bench_3_rotating_file_size ... bench:         162.22 ns/iter (+/- 21.09)
  test bench_4_rotating_daily     ... bench:         146.32 ns/iter (+/- 8.46)
  test bench_5_level_off          ... bench:           1.23 ns/iter (+/- 0.02)
  ```

<details><summary><b>Compared with other Rust crates</b></summary>

#### Disclaimer

I'm not entirely familiar with using the other Rust crates below, so if you find a bug or something worth improving in the benchmark code, feel free to open an issue to let me know.

### `tracing` (0.1.40)

```
test bench_1_file               ... bench:       2,316.25 ns/iter (+/- 107.55)
test bench_2_file_async         ... bench:         603.70 ns/iter (+/- 24.70)
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ... bench:       2,373.32 ns/iter (+/- 97.30)
test bench_5_level_off          ... bench:           0.41 ns/iter (+/- 0.00)
```

### `slog` (2.7.0)

```
test bench_1_file                     ...                   unavailable
test bench_2_file_async               ... bench:         467.90 ns/iter (+/- 4.56)
test bench_3_rotating_file_size_async ... bench:         472.49 ns/iter (+/- 17.81)
test bench_4_rotating_daily           ...                   unavailable
test bench_5_level_off                ... bench:           1.81 ns/iter (+/- 0.14)
```

### `flexi_logger` (0.29.2)

```
test bench_1_file               ... bench:       1,181.17 ns/iter (+/- 97.06)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ... bench:       1,192.97 ns/iter (+/- 44.88)
test bench_4_rotating_daily     ... bench:       1,587.54 ns/iter (+/- 59.07)
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.01)
```

### `log4rs` (1.3.0)

```
test bench_1_file               ... bench:       2,882.34 ns/iter (+/- 85.30)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ... bench:       2,990.95 ns/iter (+/- 189.15)
test bench_4_rotating_daily     ...                   unavailable
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.01)
```

### `fern` (0.6.2)

```
test bench_1_file               ... bench:       2,896.02 ns/iter (+/- 259.27)
test bench_2_file_async         ...                   unavailable
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ...                   unavailable
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.02)
```

### `ftlog` (0.2.14)

```
test bench_1_file               ...                   unavailable
test bench_2_file_async         ... bench:         254.47 ns/iter (+/- 16.07)
test bench_3_rotating_file_size ...                   unavailable
test bench_4_rotating_daily     ... bench:         253.33 ns/iter (+/- 19.89)
test bench_5_level_off          ... bench:           0.20 ns/iter (+/- 0.01)
```

### `fast_log` (1.7.4)

```
test bench_1_file                     ...                   unavailable
test bench_2_file_async               ... bench:         249.25 ns/iter (+/- 2,917.45)
test bench_3_rotating_file_size_async ... bench:         270.89 ns/iter (+/- 753.87)
test bench_4_rotating_daily_async     ... bench:         640.79 ns/iter (+/- 543.36)
test bench_5_level_off                ... bench:           0.20 ns/iter (+/- 0.02)
```
</details>

<details><summary><b>Compared with C++ spdlog</b></summary>

### `spdlog-rs` (0.4.0)

- Default features (corresponds to C++ `spdlog` using standard `<format>`)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3748502/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3790491/sec
    [info] daily_mt                       Elapsed: 0.07 secs          3815902/sec
    [info] level-off                      Elapsed: 0.00 secs        488949735/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.05 secs          5542241/sec
    [info] rotating_mt                    Elapsed: 0.06 secs          4130975/sec
    [info] daily_mt                       Elapsed: 0.06 secs          4545066/sec
    [info] level-off                      Elapsed: 0.00 secs        550055005/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 112 = 896 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.4002621 secs   2498362/sec
    [info] Elapsed: 0.3905976 secs   2560179/sec
    [info] Elapsed: 0.3966882 secs   2520871/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.0832805 secs   12007612/sec
    [info] Elapsed: 0.0836786 secs   11950486/sec
    [info] Elapsed: 0.0828995 secs   12062798/sec
    ```

- Enable `flexible-string` feature (corresponds to C++ `spdlog` using `fmt` library)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.06 secs          4402175/sec
    [info] rotating_mt                    Elapsed: 0.06 secs          4045543/sec
    [info] daily_mt                       Elapsed: 0.06 secs          4188222/sec
    [info] level-off                      Elapsed: 0.00 secs        442243056/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.05 secs          4885541/sec
    [info] rotating_mt                    Elapsed: 0.06 secs          4344448/sec
    [info] daily_mt                       Elapsed: 0.06 secs          4350549/sec
    [info] level-off                      Elapsed: 0.00 secs        514721021/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 112 = 896 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.380146 secs    2630568/sec
    [info] Elapsed: 0.3686135 secs   2712868/sec
    [info] Elapsed: 0.3628417 secs   2756022/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.0852181 secs   11734596/sec
    [info] Elapsed: 0.0864404 secs   11568664/sec
    [info] Elapsed: 0.0889495 secs   11242334/sec
    ```

### C++ `spdlog` (1.14.1)

Compiler `MSVC 19.41.34120.0`.

- Using standard `<format>`
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF -DSPDLOG_USE_STD_FORMAT=ON`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.12 secs        2,057,294/sec
    [info] rotating_mt                    Elapsed: 0.13 secs        1,878,038/sec
    [info] daily_mt                       Elapsed: 0.12 secs        2,051,127/sec
    [info] level-off                      Elapsed: 0.00 secs      151,515,151/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.18 secs        1,387,633/sec
    [info] rotating_mt                    Elapsed: 0.18 secs        1,355,687/sec
    [info] daily_mt                       Elapsed: 0.19 secs        1,347,550/sec
    [info] level-off                      Elapsed: 0.00 secs      148,086,719/sec
    ```

  - Async

    ```
    [info] -------------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 176 = 1408 KB
    [info] Total iters  : 3
    [info] -------------------------------------------------
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: block
    [info] *********************************
    [info] Elapsed: 2.8654663 secs   348983/sec
    [info] Elapsed: 2.8504903 secs   350816/sec
    [info] Elapsed: 2.8458098 secs   351393/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 1.8782355 secs   532414/sec
    [info] Elapsed: 1.8402441 secs   543406/sec
    [info] Elapsed: 1.8303429 secs   546345/sec
    ```

- Using `fmt` library
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs        3,503,397/sec
    [info] rotating_mt                    Elapsed: 0.08 secs        3,310,394/sec
    [info] daily_mt                       Elapsed: 0.08 secs        3,160,360/sec
    [info] level-off                      Elapsed: 0.00 secs      138,881,173/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.12 secs        2,091,563/sec
    [info] rotating_mt                    Elapsed: 0.13 secs        1,886,654/sec
    [info] daily_mt                       Elapsed: 0.13 secs        1,891,844/sec
    [info] level-off                      Elapsed: 0.00 secs      139,883,616/sec
    ```

  - Async

    ```
    [info] -------------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 432 = 3456 KB
    [info] Total iters  : 3
    [info] -------------------------------------------------
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: block
    [info] *********************************
    [info] Elapsed: 2.5994468 secs   384697/sec
    [info] Elapsed: 2.6129828 secs   382704/sec
    [info] Elapsed: 2.6062268 secs   383696/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 1.8697969 secs   534817/sec
    [info] Elapsed: 1.8636448 secs   536582/sec
    [info] Elapsed: 1.8804087 secs   531799/sec
    ```

</details>

[The tracking page]: https://spriteovo.github.io/spdlog-rs/dev/benchmarks/
