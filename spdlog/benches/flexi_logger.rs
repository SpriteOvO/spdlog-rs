#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf, sync::Mutex};
use test::Bencher;

use once_cell::sync::Lazy;

use flexi_logger::{
    writers::FileLogWriter, Age, Cleanup, Criterion, DeferredNow, FileSpec, LogSpecification,
    Logger, LoggerHandle, Naming, WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{info, Record};

static LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = common::BENCH_LOGS_PATH.join("flexi_logger");
    fs::create_dir_all(&path).unwrap();
    path
});

static HANDLE: Lazy<Mutex<LoggerHandle>> = Lazy::new(|| {
    Mutex::new(
        Logger::with(LogSpecification::off())
            .log_to_file(FileSpec::try_from(LOGS_PATH.join("empty.log")).unwrap())
            .write_mode(WriteMode::BufferDontFlush)
            .format(formatter)
            .start()
            .unwrap(),
    )
});

pub fn formatter(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} {}",
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        record.level(),
        &record.args()
    )
}

#[bench]
fn bench_1_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter);

    let handle = HANDLE.lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

unavailable_bench!(bench_2_file_async);

#[bench]
fn bench_3_rotating_file_size(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_file_size.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter)
        .rotate(
            Criterion::Size(common::FILE_SIZE),
            Naming::Numbers,
            Cleanup::KeepLogFiles(common::ROTATING_FILES),
        );

    let handle = HANDLE.lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_4_rotating_daily(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("rotating_daily.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Numbers,
            Cleanup::KeepLogFiles(common::ROTATING_FILES),
        );

    let handle = HANDLE.lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    HANDLE.lock().unwrap().set_new_spec(LogSpecification::off());

    bencher.iter(|| info!(bench_log_message!()))
}
