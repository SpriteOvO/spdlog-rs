#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use std::{env, thread, time::Instant};

use clap::Parser;
use spdlog::{
    formatter::{pattern, PatternFormatter},
    info,
    sink::{RotationPolicy, *},
    LevelFilter, Logger,
};
use test::black_box;

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

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

    let logger = build_test_logger(|b| {
        b.sink(
            FileSink::builder()
                .path(common::BENCH_LOGS_PATH.join("FileSink.log"))
                .truncate(true)
                .build_arc()
                .unwrap(),
        )
        .name("basic_mt")
    });
    bench_mt(logger, threads, iters);

    let logger = build_test_logger(|b| {
        b.sink(
            RotatingFileSink::builder()
                .base_path(common::BENCH_LOGS_PATH.join("RotatingFileSink_FileSize.log"))
                .rotation_policy(RotationPolicy::FileSize(FILE_SIZE))
                .max_files(ROTATING_FILES)
                .build_arc()
                .unwrap(),
        )
        .name("rotating_mt")
    });
    bench_mt(logger, threads, iters);

    let logger = build_test_logger(|b| {
        b.sink(
            RotatingFileSink::builder()
                .base_path(common::BENCH_LOGS_PATH.join("RotatingFileSink_Daily.log"))
                .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
                .build_arc()
                .unwrap(),
        )
        .name("daily_mt")
    });
    bench_mt(logger, threads, iters);

    let logger = build_test_logger(|b| b.name("level-off").level_filter(LevelFilter::Off));
    bench_mt(logger, threads, iters);
}

fn bench_mt(logger: Logger, threads_count: usize, iters: usize) {
    let start = Instant::now();

    thread::scope(|scope| {
        for _ in 0..threads_count {
            scope.spawn(|| {
                for i in 0..(iters / threads_count) {
                    info!(logger: logger, "Hello logger: msg number {}", black_box(i));
                }
            });
        }
    });

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

    let formatter = Box::new(PatternFormatter::new(pattern!(
        "[{^{level}}] {payload}{eol}"
    )));
    spdlog::default_logger()
        .sinks()
        .iter()
        .for_each(|sink| sink.set_formatter(formatter.clone()));

    bench_threaded_logging(1, args.iters);
    bench_threaded_logging(args.threads, args.iters);
}
