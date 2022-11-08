#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf};

use fern::Dispatch;
use log::info;
use once_cell::sync::Lazy;
use test::Bencher;

static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = common::BENCH_LOGS_PATH.join("fern");
    fs::create_dir_all(&path).unwrap();
    path
});

#[bench]
fn bench_1_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(path).unwrap())
        .apply()
        .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

unavailable_bench! {
    bench_2_file_async,
    bench_3_rotating_file_size,
    bench_4_rotating_daily,
    bench_5_level_off
}
