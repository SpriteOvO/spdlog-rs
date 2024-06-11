#![feature(test)]

extern crate test;

mod common;

use test::Bencher;
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, fmt::MakeWriter};

fn bench_any(
    bencher: &mut Bencher,
    writer: impl for<'writer> MakeWriter<'writer> + 'static + Send + Sync,
) {
    let _guard = tracing::dispatcher::set_default(
        &tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_writer(writer)
            .finish()
            .into(),
    );

    bencher.iter(|| info!(bench_log_message!()))
}

#[bench]
fn bench_1_file(bencher: &mut Bencher) {
    bench_any(
        bencher,
        tracing_appender::rolling::never(&*common::BENCH_LOGS_PATH, "file.log"),
    );
}

#[bench]
fn bench_2_file_async(bencher: &mut Bencher) {
    let file_appender =
        tracing_appender::rolling::never(&*common::BENCH_LOGS_PATH, "file_async.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    bench_any(bencher, non_blocking);
}

unavailable_bench!(bench_3_rotating_file_size);

#[bench]
fn bench_4_rotating_daily(bencher: &mut Bencher) {
    bench_any(
        bencher,
        tracing_appender::rolling::daily(&*common::BENCH_LOGS_PATH, "rotating_daily.log"),
    );
}

#[bench]
fn bench_5_level_off(bencher: &mut Bencher) {
    let _guard = tracing::dispatcher::set_default(
        &tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .finish()
            .into(),
    );

    bencher.iter(|| info!(bench_log_message!()))
}
