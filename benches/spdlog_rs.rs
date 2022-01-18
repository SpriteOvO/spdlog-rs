#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf, sync::Arc};
use test::Bencher;

use once_cell::sync::OnceCell;

use spdlog::{prelude::*, sink::*, LevelFilter, Logger};

fn logs_path() -> &'static PathBuf {
    static LOGS_PATH: OnceCell<PathBuf> = OnceCell::new();
    LOGS_PATH.get_or_init(|| {
        let path = common::bench_logs_path().join("spdlog_rs");
        fs::create_dir_all(&path).unwrap();
        path
    })
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = logs_path().join("file.log");

    let sink = Arc::new(FileSink::new(path, true).unwrap());
    let logger = Logger::builder().sink(sink).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let path = logs_path().join("rotating_file_size.log");

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
    let path = logs_path().join("rotating_daily.log");

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
fn bench_level_off(bencher: &mut Bencher) {
    let logger = Logger::builder().level_filter(LevelFilter::Off).build();

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}
