use std::{
    env,
    sync::{Arc, RwLock},
    time::Instant,
};

use clap::Parser;

use spdlog::{info, sink::*, Logger};

fn bench_threaded_logging(threads: usize, iters: usize) {
    info!("**********************************************************************");
    info!("Multi threaded: {} threads, {} messages", threads, iters);
    info!("**********************************************************************");

    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("logs/FileSink.log");

    let logger = Logger::builder()
        .sink(Arc::new(RwLock::new(FileSink::new(path, true).unwrap())))
        .build();
    bench_mt("FileSink (basic_mt)", &logger, threads, iters);
}

fn bench_mt(name: &str, logger: &Logger, threads_count: usize, iters: usize) {
    let start = Instant::now();

    crossbeam::thread::scope(|scope| {
        for _ in 0..threads_count {
            scope.spawn(|_| {
                for i in 0..(iters / threads_count) {
                    info!(logger: logger, "Hello logger: msg number {}", i);
                }
            });
        }
    })
    .unwrap();

    let elapsed = start.elapsed().as_secs_f64();

    info!(
        "{:<30} Elapsed: {:0.2} secs {:>16}/sec",
        name,
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

    spdlog::init();

    bench_threaded_logging(1, args.iters);
    bench_threaded_logging(args.threads, args.iters);
}
