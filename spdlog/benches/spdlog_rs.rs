#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf, sync::Arc};
use test::Bencher;

use once_cell::sync::Lazy;

use spdlog::{prelude::*, sink::*, LevelFilter, Logger};

static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = common::BENCH_LOGS_PATH.join("spdlog_rs");
    fs::create_dir_all(&path).unwrap();
    path
});

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

    let sink = Arc::new(
        FileSink::builder()
            .path(path)
            .truncate(true)
            .build()
            .unwrap(),
    );
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_file_size.log");

    let sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::FileSize(common::FILE_SIZE))
            .max_files(common::ROTATING_FILES)
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_daily(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_daily.log");

    let sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
            .max_files(common::ROTATING_FILES)
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_level_off(bencher: &mut Bencher) {
    let logger = Logger::builder().level_filter(LevelFilter::Off).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}
