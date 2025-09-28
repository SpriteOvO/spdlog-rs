#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use log::{info, LevelFilter};
use logforth::append::single_file::SingleFileBuilder;
use test::Bencher;

unavailable_bench!(bench_1_file);

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("file_async.log");

    let (single_writer, _guard) = SingleFileBuilder::new(path).build().unwrap();
    logforth::builder()
        .dispatch(|d| d.filter(LevelFilter::Info).append(single_writer))
        .apply();

    bencher.iter(|| info!(bench_log_message!()))
}
