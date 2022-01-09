#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf, sync::Arc};
use test::Bencher;

use lazy_static::lazy_static;

use spdlog::{prelude::*, sink::*, LevelFilter, Logger};

lazy_static! {
    pub static ref LOGS_PATH: PathBuf = {
        let path = common::BENCH_LOGS_PATH.join("spdlog_rs");
        fs::create_dir_all(&path).unwrap();
        path
    };
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

    let sink = Arc::new(FileSink::new(path, true).unwrap());
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_file_size.log");

    let sink = Arc::new(
        RotatingFileSink::new(
            path,
            RotationPolicy::FileSize(common::FILE_SIZE),
            common::ROTATING_FILES,
            true,
        )
        .unwrap(),
    );
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_daily(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_daily.log");

    let sink = Arc::new(
        RotatingFileSink::new(
            path,
            RotationPolicy::Daily { hour: 0, minute: 0 },
            common::ROTATING_FILES,
            true,
        )
        .unwrap(),
    );
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_no_level(bencher: &mut Bencher) {
    let logger = Logger::builder().level_filter(LevelFilter::Off).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}
