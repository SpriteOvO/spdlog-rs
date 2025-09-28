#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use std::time::Duration;

use fast_log::{
    config::Config,
    consts::LogSize,
    plugin::{
        file_split::{KeepType, Rolling, RollingType},
        packer::LogPacker,
    },
};
use log::info;
use test::Bencher;

#[bench]
fn bench_4_rotating_daily_async(bencher: &mut Bencher) {
    fast_log::init(
        Config::new()
            .file_split(
                common::BENCH_LOGS_PATH
                    .join("rotating_daily_async.log")
                    .to_str()
                    .unwrap(),
                // There is no unlimited option, so we use a large size
                Rolling::new(RollingType::BySize(LogSize::EB(1))),
                KeepType::KeepTime(Duration::from_secs(24 * 3600)),
                LogPacker {},
            )
            .chan_len(Some(100000)),
    )
    .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
