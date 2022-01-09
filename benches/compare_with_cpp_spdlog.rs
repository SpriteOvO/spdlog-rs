#![feature(test)]

extern crate test;

mod common;

use std::{env, fs, path::PathBuf, sync::Arc, time::Instant};
use test::black_box;

use clap::Parser;
use lazy_static::lazy_static;

use spdlog::{
    info,
    sink::{rotating_file_sink::RotationPolicy, *},
    LevelFilter, Logger,
};

lazy_static! {
    pub static ref LOGS_PATH: PathBuf = {
        let path = common::BENCH_LOGS_PATH.join("compare_with_cpp_spdlog");
        fs::create_dir_all(&path).unwrap();
        path
    };
}

const FILE_SIZE: u64 = 30 * 1024 * 1024;
// C++ "spdlog" is `5` here because it does not include the current file,
// "spdlog_rs" does.
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
            FileSink::new(LOGS_PATH.join("FileSink.log"), true).unwrap(),
        ))
        .name("basic_mt")
        .build();
    bench_mt(logger, threads, iters);

    let logger = Logger::builder()
        .sink(Arc::new(
            RotatingFileSink::new(
                LOGS_PATH.join("RotatingFileSink_FileSize.log"),
                RotationPolicy::FileSize(FILE_SIZE),
                ROTATING_FILES,
                false,
            )
            .unwrap(),
        ))
        .name("rotating_mt")
        .build();
    bench_mt(logger, threads, iters);

    let logger = Logger::builder()
        .sink(Arc::new(
            RotatingFileSink::new(
                LOGS_PATH.join("RotatingFileSink_Daily.log"),
                RotationPolicy::Daily { hour: 0, minute: 0 },
                0,
                false,
            )
            .unwrap(),
        ))
        .name("daily_mt")
        .build();
    bench_mt(logger, threads, iters);

    let mut logger = Logger::builder().name("level-off").build();
    logger.set_level_filter(LevelFilter::Off);
    bench_mt(logger, threads, iters);
}

fn bench_mt(logger: Logger, threads_count: usize, iters: usize) {
    let start = Instant::now();

    crossbeam::thread::scope(|scope| {
        for _ in 0..threads_count {
            scope.spawn(|_| {
                for i in 0..(iters / threads_count) {
                    black_box(info!(logger: logger, "Hello logger: msg number {}", i));
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
#[clap(name = "compare_with_cpp_spdlog")]
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
