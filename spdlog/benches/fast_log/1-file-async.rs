#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use fast_log::config::Config;
use log::info;
use test::Bencher;

unavailable_bench!(bench_1_file);

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    fast_log::init(
        Config::new()
            .file(
                common::BENCH_LOGS_PATH
                    .join("file_async.log")
                    .to_str()
                    .unwrap(),
            )
            .chan_len(Some(1000000)),
    )
    .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
