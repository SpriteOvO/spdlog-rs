#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use ftlog::appender::FileAppender;
use log::info;
use test::Bencher;

unavailable_bench!(bench_1_file);

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("file_async.log");

    let _guard = ftlog::builder()
        .root(FileAppender::builder().path(path).build())
        .try_init()
        .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

unavailable_bench!(bench_3_rotating_file_size);
