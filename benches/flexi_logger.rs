#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf, sync::Mutex};
use test::Bencher;

use once_cell::sync::OnceCell;

use flexi_logger::{
    writers::FileLogWriter, Age, Cleanup, Criterion, DeferredNow, FileSpec, LogSpecification,
    Logger, LoggerHandle, Naming, WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{info, Record};

fn logs_path() -> &'static PathBuf {
    static LOGS_PATH: OnceCell<PathBuf> = OnceCell::new();
    LOGS_PATH.get_or_init(|| {
        let path = common::bench_logs_path().join("flexi_logger");
        fs::create_dir_all(&path).unwrap();
        path
    })
}

fn handle() -> &'static Mutex<LoggerHandle> {
    static HANDLE: OnceCell<Mutex<LoggerHandle>> = OnceCell::new();
    HANDLE.get_or_init(|| {
        Mutex::new(
            Logger::with(LogSpecification::off())
                .log_to_file(FileSpec::try_from(logs_path().join("empty.log")).unwrap())
                .write_mode(WriteMode::BufferDontFlush)
                .format(formatter)
                .start()
                .unwrap(),
        )
    })
}

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
fn bench_file(bencher: &mut Bencher) {
    let path = logs_path().join("file.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter);

    let mut handle = handle().lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let path = logs_path().join("rotating_file_size.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter)
        .rotate(
            Criterion::Size(common::FILE_SIZE),
            Naming::Numbers,
            Cleanup::KeepLogFiles(common::ROTATING_FILES),
        );

    let mut handle = handle().lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_rotating_daily(bencher: &mut Bencher) {
    let path = logs_path().join("rotating_daily.log");

    let writer_builder = FileLogWriter::builder(FileSpec::try_from(path).unwrap())
        .write_mode(WriteMode::BufferDontFlush)
        .format(formatter)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Numbers,
            Cleanup::KeepLogFiles(common::ROTATING_FILES),
        );

    let mut handle = handle().lock().unwrap();
    handle.set_new_spec(LogSpecification::info());
    handle.reset_flw(&writer_builder).unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_level_off(bencher: &mut Bencher) {
    handle()
        .lock()
        .unwrap()
        .set_new_spec(LogSpecification::off());

    bencher.iter(|| info!(bench_log_message!()))
}
