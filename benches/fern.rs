#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf};
use test::Bencher;

use once_cell::sync::OnceCell;

use fern::Dispatch;
use log::info;

fn logs_path() -> &'static PathBuf {
    static LOGS_PATH: OnceCell<PathBuf> = OnceCell::new();
    LOGS_PATH.get_or_init(|| {
        let path = common::bench_logs_path().join("fern");
        fs::create_dir_all(&path).unwrap();
        path
    })
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = logs_path().join("file.log");

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
