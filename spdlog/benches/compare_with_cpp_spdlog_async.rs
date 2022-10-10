#![feature(test)]

extern crate test;

mod common;

use std::{cmp, env, fs, path::PathBuf, sync::Arc, thread, time::Instant};
use test::black_box;

use clap::Parser;
use once_cell::sync::Lazy;

use spdlog::{
    error::{Error, SendToChannelError},
    prelude::*,
    sink::*,
    ThreadPool,
};

required_multi_thread_feature!();

static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = common::BENCH_LOGS_PATH.join("compare_with_cpp_spdlog_async");
    fs::create_dir_all(&path).unwrap();
    path
});

fn bench(
    policy: OverflowPolicy,
    file_name: &str,
    message_count: usize,
    queue_size: usize,
    threads: usize,
    iters: usize,
) {
    info!("");
    info!("********************************************");
    info!("Queue Overflow Policy: {policy:?}");
    info!("********************************************");

    for _ in 0..iters {
        let thread_pool = Arc::new(ThreadPool::builder().capacity(queue_size).build().unwrap());

        let file_sink = Arc::new(
            FileSink::builder()
                .path(LOGS_PATH.join(file_name))
                .truncate(true)
                .build()
                .unwrap(),
        );

        let async_sink = Arc::new(
            AsyncPoolSink::builder()
                .thread_pool(thread_pool)
                .overflow_policy(policy)
                .sink(file_sink)
                .error_handler(|err| panic!("an error occurred: {err}"))
                .build()
                .unwrap(),
        );

        let logger = Logger::builder()
            .sink(async_sink)
            .name("async_logger")
            .error_handler(|err| {
                if let Error::SendToChannel(SendToChannelError::Full, _dropped_data) = err {
                    // ignore
                } else {
                    panic!("an error occurred: {err}")
                }
            })
            .build()
            .unwrap();

        bench_mt(logger, message_count, threads);
    }
}

fn bench_mt(logger: Logger, message_count: usize, threads: usize) {
    let msgs_per_thread = message_count / threads;
    let msgs_per_thread_mod = message_count % threads;

    let start = Instant::now();

    thread::scope(|scope| {
        for t in 0..threads {
            let logger = &logger;
            let message_count = if t == 0 && msgs_per_thread_mod != 0 {
                msgs_per_thread + msgs_per_thread_mod
            } else {
                msgs_per_thread
            };
            scope.spawn(move || {
                for i in 0..message_count {
                    info!(logger: logger, "Hello logger: msg number {}", black_box(i));
                }
            });
        }
    });

    let elapsed = start.elapsed().as_secs_f64();

    info!(
        "Elapsed: {} secs\t {}/sec",
        elapsed,
        (message_count as f64 / elapsed) as usize
    );
}

/// A benchmark for comparing with the C++ logging library spdlog (async
/// version).
#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_CRATE_NAME"))]
struct Args {
    /// Number of the benchmark messages
    #[clap(long, default_value_t = 1000000)]
    message_count: usize,

    /// Number of the channel capacity
    /// [default: `min(message_count + 2, 8192)`]
    #[clap(long, validator(arg_queue_size_validator))]
    queue_size: Option<usize>,

    /// Number of the benchmark threads
    #[clap(long, default_value_t = 10)]
    threads: usize,

    /// Number of the benchmark iterations
    #[clap(long, default_value_t = 3)]
    iters: usize,
}

fn arg_queue_size_validator(
    input: &str,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let size = input.parse()?;
    if size > 500000 {
        Err("max queue size allowed: 500,000".into())
    } else {
        Ok(size)
    }
}

fn main() {
    let mut args = Args::parse_from(env::args().filter(|arg| arg != "--bench"));
    if args.queue_size.is_none() {
        args.queue_size = Some(cmp::min(args.message_count + 2, 8192));
    }

    const SLOT_SIZE: usize = spdlog::RecordOwned::__SIZE_OF;
    let queue_size = args.queue_size.unwrap();

    info!("--------------------------------------------");
    info!("Messages     : {}", args.message_count);
    info!("Threads      : {}", args.threads);
    info!("Queue        : {} slots", queue_size);
    info!(
        "Queue memory : {} x {} = {} KB",
        queue_size,
        SLOT_SIZE,
        (queue_size * SLOT_SIZE) / 1024
    );
    info!("Total iters  : {}", args.iters);
    info!("--------------------------------------------");

    bench(
        OverflowPolicy::Block,
        "basic_async.log",
        args.message_count,
        queue_size,
        args.threads,
        args.iters,
    );
    bench(
        OverflowPolicy::DropIncoming,
        "basic_async-drop-incoming.log",
        args.message_count,
        queue_size,
        args.threads,
        args.iters,
    );
}
