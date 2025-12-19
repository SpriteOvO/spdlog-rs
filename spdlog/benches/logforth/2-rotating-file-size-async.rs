#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use log::{info, LevelFilter};
use logforth::append::rolling_file::{RollingFileBuilder, Rotation};
use test::Bencher;

#[bench]
fn bench_3_rotating_file_size_async(bencher: &mut Bencher) {
    let (single_writer, _guard) = RollingFileBuilder::new(&*common::BENCH_LOGS_PATH)
        .rotation(Rotation::Never)
        .max_file_size(common::FILE_SIZE as usize)
        .filename_prefix("rotating_file_size_async")
        .build()
        .unwrap();
    logforth::builder()
        .dispatch(|d| d.filter(LevelFilter::Info).append(single_writer))
        .apply();

    bencher.iter(|| info!(bench_log_message!()))
}
