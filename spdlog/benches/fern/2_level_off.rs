#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use fern::Dispatch;
use log::info;
use test::Bencher;

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("level_off.log");

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
        .level(log::LevelFilter::Off)
        .chain(fern::log_file(path).unwrap())
        .apply()
        .unwrap();

    bencher.iter(|| info!(bench_log_message!()))
}
