#![feature(test)]

extern crate test;

mod common;

use std::{fs, path::PathBuf};
use test::Bencher;

use lazy_static::lazy_static;

use log::{info, LevelFilter};
use log4rs::{
    append::{
        file::FileAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller,
                {trigger::size::SizeTrigger, CompoundPolicy},
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

lazy_static! {
    pub static ref LOGS_PATH: PathBuf = {
        let path = common::BENCH_LOGS_PATH.join("log4rs");
        fs::create_dir_all(&path).unwrap();
        path
    };
    pub static ref HANDLE: Handle = log4rs::init_config(
        Config::builder()
            .build(Root::builder().build(LevelFilter::Off))
            .unwrap()
    )
    .unwrap();
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    let path = LOGS_PATH.join("file.log");

    let appender = FileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}\n")))
        .build(path)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    HANDLE.set_config(config);

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_rotating_file_size(bencher: &mut Bencher) {
    let pattern_path = LOGS_PATH.join("rotating_file_size_{}.log");
    let path = LOGS_PATH.join("rotating_file_size.log");

    let policy = CompoundPolicy::new(
        Box::new(SizeTrigger::new(common::FILE_SIZE)),
        Box::new(
            FixedWindowRoller::builder()
                .build(
                    pattern_path.to_str().unwrap(),
                    common::ROTATING_FILES as u32,
                )
                .unwrap(),
        ),
    );
    let appender = RollingFileAppender::builder()
        .append(false)
        .encoder(Box::new(PatternEncoder::new("{d} {l} {m}\n")))
        .build(path, Box::new(policy))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("rotating_file_size", Box::new(appender)))
        .build(
            Root::builder()
                .appender("rotating_file_size")
                .build(LevelFilter::Info),
        )
        .unwrap();

    HANDLE.set_config(config);

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_level_off(bencher: &mut Bencher) {
    let config = Config::builder()
        .build(Root::builder().build(LevelFilter::Off))
        .unwrap();

    HANDLE.set_config(config);

    bencher.iter(|| info!(bench_log_message!()))
}
