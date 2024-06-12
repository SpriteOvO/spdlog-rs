#![feature(test)]

extern crate test;

#[path = "../common/mod.rs"]
mod common;

use fern::Dispatch;
use log::info;
use test::Bencher;

#[bench]
fn bench_1_file(bencher: &mut Bencher) {
    let path = common::BENCH_LOGS_PATH.join("file.log");

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
}
