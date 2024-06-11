#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use ftlog::{appender::FileAppender, LevelFilter};
use log::info;
use test::Bencher;

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("level_off.log");

    let _guard = ftlog::builder()
        .root(FileAppender::builder().path(path).build())
        .max_log_level(LevelFilter::Off)
        .try_init()
        .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
