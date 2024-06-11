#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use ftlog::appender::{FileAppender, Period};
use log::info;
use test::Bencher;

#[bench]
fn bench_4_rotating_daily(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("rotating_daily.log");

    let _guard = ftlog::builder()
        .root(
            FileAppender::builder()
                .path(path)
                .rotate(Period::Day)
                .build(),
        )
        .try_init()
        .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
