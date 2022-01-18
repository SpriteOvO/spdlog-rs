#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf};
use test::Bencher;

use once_cell::sync::OnceCell;

use slog::{info, o, Fuse, Logger};
use sloggers::{file::FileLoggerBuilder, types::SourceLocation, Build};

fn logs_path() -> &'static PathBuf {
    static LOGS_PATH: OnceCell<PathBuf> = OnceCell::new();
    LOGS_PATH.get_or_init(|| {
        let path = common::bench_logs_path().join("slog");
        fs::create_dir_all(&path).unwrap();
        path
    })
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = logs_path().join("file.log");

    let drain = Fuse(
        FileLoggerBuilder::new(path)
            .truncate()
            .source_location(SourceLocation::None)
            .build()
            .unwrap(),
    );
    let logger = Logger::root(drain, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let path = logs_path().join("rotating_file_size.log");

    let drain = Fuse(
        FileLoggerBuilder::new(path)
            .truncate()
            .source_location(SourceLocation::None)
            .rotate_size(common::FILE_SIZE)
            .rotate_keep(common::ROTATING_FILES)
            .build()
            .unwrap(),
    );
    let logger = Logger::root(drain, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}

#[bench]
fn bench_level_off(bencher: &mut Bencher) {
    let logger = Logger::root(slog::Discard, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}
