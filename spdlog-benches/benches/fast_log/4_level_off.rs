#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use fast_log::config::Config;
use log::{info, LevelFilter};
use test::Bencher;

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    fast_log::init(
        Config::new()
            .file(
                common::BENCH_LOGS_PATH
                    .join("levL_off.log")
                    .to_str()
                    .unwrap(),
            )
            .chan_len(Some(1000000))
            .level(LevelFilter::Off),
    )
    .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
