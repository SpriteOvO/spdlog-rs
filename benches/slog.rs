#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf};
use test::Bencher;

use lazy_static::lazy_static;

use slog::{info, o, Fuse, Logger};
use sloggers::{file::FileLoggerBuilder, types::SourceLocation, Build};

lazy_static! {
    pub static ref LOGS_PATH: PathBuf = {
        let path = common::BENCH_LOGS_PATH.join("slog");
        fs::create_dir_all(&path).unwrap();
        path
    };
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

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
    let path = LOGS_PATH.join("rotating_file_size.log");

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
fn bench_no_level(bencher: &mut Bencher) {
    let logger = Logger::root(slog::Discard, o!());

    bencher.iter(|| info!(logger, bench_log_message!()))
}
