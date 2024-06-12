# Benchmarks

[The tracking page] for benchmark changes for each commit during development.

Run `cargo +nightly bench --features multi-thread` in the root directory of this repository for benchmarking.

The following results are generated with `Windows 10 64 bit` and `Intel i9-10900KF CPU @ 3.70GHz`.

### `spdlog-rs` (0.3.0)

- Default features

  ```
  test bench_1_file               ... bench:         208 ns/iter (+/- 16)
  test bench_2_file_async         ... bench:         178 ns/iter (+/- 8)
  test bench_3_rotating_file_size ... bench:         211 ns/iter (+/- 37)
  test bench_4_rotating_daily     ... bench:         211 ns/iter (+/- 22)
  test bench_5_level_off          ... bench:           1 ns/iter (+/- 0)
  ```

- Enable `flexible-string` feature

  ```
  test bench_1_file               ... bench:         179 ns/iter (+/- 16)
  test bench_2_file_async         ... bench:         179 ns/iter (+/- 10)
  test bench_3_rotating_file_size ... bench:         183 ns/iter (+/- 23)
  test bench_4_rotating_daily     ... bench:         189 ns/iter (+/- 18)
  test bench_5_level_off          ... bench:           1 ns/iter (+/- 0)
  ```

<details><summary><b>Compared with other Rust crates</b></summary>

#### Disclaimer

I'm not entirely familiar with using the other Rust crates below, so if you find a bug or something worth improving in the benchmark code, feel free to open an issue to let me know.

### `tracing` (0.1.37)

```
test bench_1_file               ... bench:       2,387 ns/iter (+/- 165)
test bench_2_file_async         ... bench:         767 ns/iter (+/- 161)
test bench_3_rotating_file_size ...                unavailable
test bench_4_rotating_daily     ... bench:       2,395 ns/iter (+/- 107)
test bench_5_level_off          ... bench:           0 ns/iter (+/- 0)
```

### `slog` (2.7.0)

```
test bench_1_file                     ...                unavailable
test bench_2_file_async               ... bench:         464 ns/iter (+/- 20)
test bench_3_rotating_file_size_async ... bench:         463 ns/iter (+/- 20)
test bench_4_rotating_daily           ...                unavailable
test bench_5_level_off                ... bench:           2 ns/iter (+/- 0)
```

### `flexi_logger` (0.24.1)

```
test bench_1_file               ... bench:       1,732 ns/iter (+/- 121)
test bench_2_file_async         ...                unavailable
test bench_3_rotating_file_size ... bench:       1,701 ns/iter (+/- 110)
test bench_4_rotating_daily     ... bench:       2,486 ns/iter (+/- 191)
test bench_5_level_off          ... bench:           0 ns/iter (+/- 0)
```

### `log4rs` (1.2.0)

```
test bench_1_file               ... bench:       3,350 ns/iter (+/- 254)
test bench_2_file_async         ...                unavailable
test bench_3_rotating_file_size ... bench:       3,320 ns/iter (+/- 236)
test bench_4_rotating_daily     ...                unavailable
test bench_5_level_off          ... bench:           0 ns/iter (+/- 0)
```

### `fern` (0.6.1)

```
test bench_1_file               ... bench:       3,337 ns/iter (+/- 203)
test bench_2_file_async         ...                unavailable
test bench_3_rotating_file_size ...                unavailable
test bench_4_rotating_daily     ...                unavailable
test bench_5_level_off          ...                unavailable
```
</details>

<details><summary><b>Compared with C++ spdlog</b></summary>

### `spdlog-rs` (0.3.0)

- Default features (corresponds to C++ `spdlog` using standard `<format>`)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.09 secs          2881439/sec
    [info] rotating_mt                    Elapsed: 0.09 secs          2810747/sec
    [info] daily_mt                       Elapsed: 0.09 secs          2829965/sec
    [info] level-off                      Elapsed: 0.00 secs        407365162/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3803015/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3792993/sec
    [info] daily_mt                       Elapsed: 0.07 secs          3739458/sec
    [info] level-off                      Elapsed: 0.00 secs        609756097/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 104 = 832 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.6017485 secs   1661823/sec
    [info] Elapsed: 0.6075015 secs   1646086/sec
    [info] Elapsed: 0.5799057 secs   1724418/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.0794148 secs   12592111/sec
    [info] Elapsed: 0.0818668 secs   12214963/sec
    [info] Elapsed: 0.0784296 secs   12750288/sec
    ```

- Enable `flexible-string` feature (corresponds to C++ `spdlog` using `fmt` library)

  - Sync

    ```
    [info] **********************************************************************
    [info] Multi threaded: 1 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.08 secs          2969371/sec
    [info] rotating_mt                    Elapsed: 0.08 secs          2952681/sec
    [info] daily_mt                       Elapsed: 0.08 secs          2992041/sec
    [info] level-off                      Elapsed: 0.00 secs        514721021/sec
    [info] **********************************************************************
    [info] Multi threaded: 4 threads, 250000 messages
    [info] **********************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs          3496977/sec
    [info] rotating_mt                    Elapsed: 0.07 secs          3399459/sec
    [info] daily_mt                       Elapsed: 0.07 secs          3364660/sec
    [info] level-off                      Elapsed: 0.00 secs        632751202/sec
    ```

  - Async

    ```
    [info] --------------------------------------------
    [info] Messages     : 1000000
    [info] Threads      : 10
    [info] Queue        : 8192 slots
    [info] Queue memory : 8192 x 104 = 832 KB
    [info] Total iters  : 3
    [info] --------------------------------------------
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: Block
    [info] ********************************************
    [info] Elapsed: 0.5647074 secs   1770828/sec
    [info] Elapsed: 0.5635446 secs   1774482/sec
    [info] Elapsed: 0.5546488 secs   1802942/sec
    [info]
    [info] ********************************************
    [info] Queue Overflow Policy: DropIncoming
    [info] ********************************************
    [info] Elapsed: 0.0824193 secs   12133080/sec
    [info] Elapsed: 0.0794965 secs   12579170/sec
    [info] Elapsed: 0.0847295 secs   11802264/sec
    ```

### C++ `spdlog` (1.11.0)

- Using standard `<format>`
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF -DSPDLOG_USE_STD_FORMAT=ON`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.13 secs        1,906,872/sec
    [info] rotating_mt                    Elapsed: 0.13 secs        1,866,094/sec
    [info] daily_mt                       Elapsed: 0.13 secs        1,981,681/sec
    [info] level-off                      Elapsed: 0.00 secs      147,466,525/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.19 secs        1,346,298/sec
    [info] rotating_mt                    Elapsed: 0.20 secs        1,252,840/sec
    [info] daily_mt                       Elapsed: 0.20 secs        1,263,637/sec
    [info] level-off                      Elapsed: 0.00 secs      148,456,057/sec
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
    [info] Elapsed: 2.2210225 secs   450243/sec
    [info] Elapsed: 2.2159465 secs   451274/sec
    [info] Elapsed: 2.1952666 secs   455525/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 0.5903473 secs   1693918/sec
    [info] Elapsed: 0.6034908 secs   1657026/sec
    [info] Elapsed: 0.6127076 secs   1632099/sec
    ```

- Using `fmt` library
  (compiled with `cmake -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_STANDARD=20 -DCMAKE_CXX_STANDARD_REQUIRED=ON -DSPDLOG_BUILD_BENCH=ON -DSPDLOG_BUILD_EXAMPLE=OFF`)

  - Sync

    ```
    [info] **************************************************************
    [info] Multi threaded: 1 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.07 secs        3,430,498/sec
    [info] rotating_mt                    Elapsed: 0.07 secs        3,481,225/sec
    [info] daily_mt                       Elapsed: 0.07 secs        3,563,268/sec
    [info] level-off                      Elapsed: 0.00 secs      138,757,839/sec
    [info] **************************************************************
    [info] Multi threaded: 4 threads, 250,000 messages
    [info] **************************************************************
    [info] basic_mt                       Elapsed: 0.11 secs        2,214,753/sec
    [info] rotating_mt                    Elapsed: 0.12 secs        2,059,991/sec
    [info] daily_mt                       Elapsed: 0.12 secs        2,058,245/sec
    [info] level-off                      Elapsed: 0.00 secs      139,938,427/sec
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
    [info] Elapsed: 2.0660842 secs   484007/sec
    [info] Elapsed: 2.0782232 secs   481180/sec
    [info] Elapsed: 2.0823594 secs   480224/sec
    [info]
    [info] *********************************
    [info] Queue Overflow Policy: overrun
    [info] *********************************
    [info] Elapsed: 0.3435961 secs   2910393/sec
    [info] Elapsed: 0.3418817 secs   2924988/sec
    [info] Elapsed: 0.3393879 secs   2946481/sec
    ```

</details>

[The tracking page]: https://spriteovo.github.io/spdlog-rs/dev/benchmarks/
[`4cfdc8c`]: https://github.com/gabime/spdlog/commit/4cfdc8c5c84f696774cb9acde2f95c9e87c11a5e
