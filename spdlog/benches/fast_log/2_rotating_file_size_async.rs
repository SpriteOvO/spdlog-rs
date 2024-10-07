#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use fast_log::{config::Config, consts::LogSize};
use log::info;
use test::Bencher;

#[bench]
fn bench_3_rotating_file_size_async(bencher: &mut Bencher) {
    fast_log::init(
        Config::new()
            .file_loop(
                common::BENCH_LOGS_PATH
                    .join("rotating_file_size_async.log")
                    .to_str()
                    .unwrap(),
                LogSize::B(common::FILE_SIZE as usize),
            )
            .chan_len(Some(100000)),
    )
    .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
