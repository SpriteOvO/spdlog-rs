#![feature(test)]

extern crate test;

mod common;

use std::{env, fs, path::PathBuf, sync::Arc, time::Instant};
use test::black_box;

use clap::Parser;
use once_cell::sync::Lazy;

use spdlog::{
    info,
    sink::{RotationPolicy, *},
    LevelFilter, Logger,
};

static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = common::BENCH_LOGS_PATH.join("compare_with_cpp_spdlog");
    fs::create_dir_all(&path).unwrap();
    path
});

const FILE_SIZE: u64 = 30 * 1024 * 1024;
// C++ "spdlog" is `5` here because it does not include the current file,
// "spdlog-rs" does.
//
// However, this actually barely affects the benchmark results, because the data
// that will be written is not enough to rotate 5 times.
const ROTATING_FILES: usize = 6;

fn bench_threaded_logging(threads: usize, iters: usize) {
    info!("**********************************************************************");
    info!("Multi threaded: {} threads, {} messages", threads, iters);
    info!("**********************************************************************");

    let logger = Logger::builder()
        .sink(Arc::new(
            FileSink::builder()
                .path(LOGS_PATH.join("FileSink.log"))
                .truncate(true)
                .build()
                .unwrap(),
        ))
        .name("basic_mt")
        .build();
    bench_mt(logger, threads, iters);

    let logger = Logger::builder()
        .sink(Arc::new(
            RotatingFileSink::builder()
                .base_path(LOGS_PATH.join("RotatingFileSink_FileSize.log"))
                .rotation_policy(RotationPolicy::FileSize(FILE_SIZE))
                .max_files(ROTATING_FILES)
                .build()
                .unwrap(),
        ))
        .name("rotating_mt")
        .build();
    bench_mt(logger, threads, iters);

    let logger = Logger::builder()
        .sink(Arc::new(
            RotatingFileSink::builder()
                .base_path(LOGS_PATH.join("RotatingFileSink_Daily.log"))
                .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
                .build()
                .unwrap(),
        ))
        .name("daily_mt")
        .build();
    bench_mt(logger, threads, iters);

    let logger = Logger::builder()
        .name("level-off")
        .level_filter(LevelFilter::Off)
        .build();
    bench_mt(logger, threads, iters);
}

fn bench_mt(logger: Logger, threads_count: usize, iters: usize) {
    let start = Instant::now();

    crossbeam::thread::scope(|scope| {
        for _ in 0..threads_count {
            scope.spawn(|_| {
                for i in 0..(iters / threads_count) {
                    info!(logger: logger, "Hello logger: msg number {}", black_box(i));
                }
            });
        }
    })
    .unwrap();

    let elapsed = start.elapsed().as_secs_f64();

    info!(
        "{:<30} Elapsed: {:0.2} secs {:>16}/sec",
        logger.name().unwrap(),
        elapsed,
        (iters as f64 / elapsed) as usize
    );
}

/// A benchmark for comparing with the C++ logging library spdlog.
#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_CRATE_NAME"))]
struct Args {
    /// Number of the benchmark threads
    #[clap(long, default_value_t = 4)]
    threads: usize,

    /// Number of the benchmark iterations
    #[clap(long, default_value_t = 250000)]
    iters: usize,
}

fn main() {
    let args = Args::parse_from(env::args().filter(|arg| arg != "--bench"));

    bench_threaded_logging(1, args.iters);
    bench_threaded_logging(args.threads, args.iters);
}
