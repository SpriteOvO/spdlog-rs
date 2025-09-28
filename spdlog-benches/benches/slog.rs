#![feature(test)]

extern crate test;

mod common;

use slog::{info, o, Fuse, Logger};
use sloggers::{file::FileLoggerBuilder, types::SourceLocation, Build};
use test::Bencher;

unavailable_bench!(bench_1_file);

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("file_async.log");

    let drain = Fuse(
        FileLoggerBuilder::new(path)
            .truncate()
            .source_location(SourceLocation::None)
            .build()
            .unwrap(),
    );
    let logger = Logger::root(drain, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}

#[bench]
fn bench_3_rotating_file_size_async(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("rotating_file_size_async.log");

    let drain = Fuse(
        FileLoggerBuilder::new(path)
            .truncate()
            .source_location(SourceLocation::None)
            .rotate_size(common::FILE_SIZE)
            .rotate_keep(common::ROTATING_FILES)
            .build()
            .unwrap(),
    );
    let logger = Logger::root(drain, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}

unavailable_bench! {
    bench_4_rotating_daily,
}

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let logger = Logger::root(slog::Discard, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}
