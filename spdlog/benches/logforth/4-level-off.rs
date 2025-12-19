#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use log::{info, LevelFilter};
use logforth::append::single_file::SingleFileBuilder;
use test::Bencher;

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("level_off.log");

    let (single_writer, _guard) = SingleFileBuilder::new(path).build().unwrap();
    logforth::builder()
        .dispatch(|d| d.filter(LevelFilter::Off).append(single_writer))
        .apply();

    bencher.iter(|| info!(bench_log_message!()))
}
