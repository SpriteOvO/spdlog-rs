#![feature(test)]

extern crate test;

mod common;

use std::{path::PathBuf, sync::Arc};

use spdlog::{
    error::{Error, ErrorHandler, SendToChannelError},
    prelude::*,
    sink::*,
    ThreadPool,
};
use test::Bencher;

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

required_multi_thread_feature!();

enum Mode {
    Sync,
    Async,
}

impl Mode {
    fn path(&self, file_name: &str) -> PathBuf {
        let file_name = match self {
            Self::Sync => format!("{file_name}.log"),
            Self::Async => format!("{file_name}_async.log"),
        };
        common::BENCH_LOGS_PATH.join(file_name)
    }

    fn final_sink(&self, sink: Arc<dyn Sink>) -> Arc<dyn Sink> {
        match self {
            Self::Sync => sink,
            Self::Async => {
                let thread_pool = Arc::new(ThreadPool::builder().build().unwrap());

                Arc::new(
                    AsyncPoolSink::builder()
                        .thread_pool(thread_pool)
                        .overflow_policy(OverflowPolicy::DropIncoming)
                        .sink(sink)
                        .build()
                        .unwrap(),
                )
            }
        }
    }

    fn error_handler(&self) -> ErrorHandler {
        const PANIC_ERR: fn(Error) = |err| panic!("an error occurred: {err}");

        match self {
            Self::Sync => PANIC_ERR,
            Self::Async => move |err| {
                if let Error::SendToChannel(SendToChannelError::Full, _dropped_data) = err {
                    // ignore
                } else {
                    PANIC_ERR(err);
                }
            },
        }
    }
}

fn bench_any(bencher: &mut Bencher, mode: Mode, sink: Arc<dyn Sink>) {
    sink.set_error_handler(Some(|err| panic!("an error occurred: {err}")));

    let logger = build_test_logger(|b| {
        b.error_handler(mode.error_handler())
            .sink(mode.final_sink(sink))
    });

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}

fn bench_file_inner(bencher: &mut Bencher, mode: Mode) {
    let sink: Arc<dyn Sink> = Arc::new(
        FileSink::builder()
            .path(mode.path("file"))
            .truncate(true)
            .build()
            .unwrap(),
    );
    bench_any(bencher, mode, sink);
}

fn bench_rotating_inner(bencher: &mut Bencher, rotation_policy: RotationPolicy) {
    let sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(Mode::Sync.path(match rotation_policy {
                RotationPolicy::FileSize(_) => "rotating_file_size",
                RotationPolicy::Daily { .. } => "rotating_daily",
                RotationPolicy::Hourly => "rotating_hourly",
            }))
            .rotation_policy(rotation_policy)
            .max_files(common::ROTATING_FILES)
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );
    bench_any(bencher, Mode::Sync, sink);
}

#[bench]
fn bench_1_file(bencher: &mut Bencher) {
    bench_file_inner(bencher, Mode::Sync);
}

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    bench_file_inner(bencher, Mode::Async);
}

#[bench]
fn bench_3_rotating_file_size(bencher: &mut Bencher) {
    bench_rotating_inner(bencher, RotationPolicy::FileSize(common::FILE_SIZE));
}

#[bench]
fn bench_4_rotating_daily(bencher: &mut Bencher) {
    bench_rotating_inner(bencher, RotationPolicy::Daily { hour: 0, minute: 0 });
}

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let logger = build_test_logger(|b| b.level_filter(LevelFilter::Off));

    bencher.iter(|| info!(logger: logger, bench_log_message!()))
}
