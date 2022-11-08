#![feature(test)]

extern crate test;

use std::{cell::RefCell, sync::Arc};

use spdlog::{
    formatter::{pattern, Formatter, FullFormatter, Pattern, PatternFormatter},
    prelude::*,
    sink::{Sink, WriteSink},
    Record, StringBuf,
};
use test::Bencher;

#[derive(Clone)]
struct BenchSink<F> {
    formatter: F,
    buffer: RefCell<StringBuf>,
}

impl<F: Formatter> BenchSink<F> {
    fn new(formatter: F) -> Self {
        Self {
            formatter,
            buffer: RefCell::new(StringBuf::with_capacity(512)),
        }
    }
}

// I think we're just testing benchmarks here, and they should not be executed
// in parallel, so the data race from `buffer` shouldn't be an problem?
unsafe impl<F> Sync for BenchSink<F> {}

impl<F: Formatter> Sink for BenchSink<F> {
    fn log(&self, record: &Record) -> spdlog::Result<()> {
        self.formatter
            .format(record, &mut self.buffer.borrow_mut())?;
        Ok(())
    }

    fn flush(&self) -> spdlog::Result<()> {
        unimplemented!()
    }

    fn level_filter(&self) -> spdlog::LevelFilter {
        unimplemented!()
    }

    fn set_level_filter(&self, _level_filter: spdlog::LevelFilter) {
        unimplemented!()
    }

    fn set_formatter(&self, _formatter: Box<dyn Formatter>) {
        unimplemented!()
    }

    fn set_error_handler(&self, _handler: Option<spdlog::ErrorHandler>) {
        unimplemented!()
    }
}

fn bench_formatter(bencher: &mut Bencher, formatter: impl Formatter + 'static) {
    let bench_sink = Arc::new(BenchSink::new(formatter));
    let logger = Logger::builder().sink(bench_sink).build().unwrap();

    bencher.iter(|| info!(logger: logger, "payload"));
}

fn bench_pattern(bencher: &mut Bencher, pattern: impl Pattern + Clone + 'static) {
    bench_formatter(bencher, PatternFormatter::new(pattern));
}

#[bench]
fn bench_1_full_formatter(bencher: &mut Bencher) {
    bench_formatter(bencher, FullFormatter::new())
}

#[bench]
fn bench_2_full_pattern(bencher: &mut Bencher) {
    let pattern = pattern!("[{date} {time}.{millisecond}] [{level}] {payload}{eol}");

    let full_formatter = Arc::new(
        WriteSink::builder()
            .formatter(Box::new(FullFormatter::new()))
            .target(Vec::new())
            .build()
            .unwrap(),
    );

    let full_pattern = Arc::new(
        WriteSink::builder()
            .formatter(Box::new(PatternFormatter::new(pattern.clone())))
            .target(Vec::new())
            .build()
            .unwrap(),
    );

    let combination = Logger::builder()
        .sink(full_formatter.clone())
        .sink(full_pattern.clone())
        .build()
        .unwrap();

    info!(logger: combination, "test payload");

    assert_eq!(
        String::from_utf8(full_formatter.clone_target()).unwrap(),
        String::from_utf8(full_pattern.clone_target()).unwrap()
    );

    bench_pattern(bencher, pattern)
}

#[bench]
fn bench_weekday_name(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{weekday_name}"))
}

#[bench]
fn bench_weekday_name_full(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{weekday_name_full}"))
}

#[bench]
fn bench_month_name(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{month_name}"))
}

#[bench]
fn bench_month_name_full(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{month_name_full}"))
}

#[bench]
fn bench_datetime(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{datetime}"))
}

#[bench]
fn bench_year_short(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{year_short}"))
}

#[bench]
fn bench_year(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{year}"))
}

#[bench]
fn bench_date_short(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{date_short}"))
}

#[bench]
fn bench_date(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{date}"))
}

#[bench]
fn bench_month(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{month}"))
}

#[bench]
fn bench_day(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{day}"))
}

#[bench]
fn bench_hour(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{hour}"))
}

#[bench]
fn bench_hour_12(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{hour_12}"))
}

#[bench]
fn bench_minute(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{minute}"))
}

#[bench]
fn bench_second(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{second}"))
}

#[bench]
fn bench_millsecond(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{millisecond}"))
}

#[bench]
fn bench_microsecond(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{microsecond}"))
}

#[bench]
fn bench_nanosecond(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{nanosecond}"))
}

#[bench]
fn bench_am_pm(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{am_pm}"))
}

#[bench]
fn bench_time_12(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{time_12}"))
}

#[bench]
fn bench_time_short(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{time_short}"))
}

#[bench]
fn bench_time(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{time}"))
}

#[bench]
fn bench_tz_offset(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{tz_offset}"))
}

#[bench]
fn bench_unix_timestamp(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{unix_timestamp}"))
}

#[bench]
fn bench_full(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{full}"))
}

#[bench]
fn bench_level(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{level}"))
}

#[bench]
fn bench_level_short(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{level_short}"))
}

#[bench]
fn bench_source(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{source}"))
}

#[bench]
fn bench_file_name(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{file_name}"))
}

#[bench]
fn bench_file(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{file}"))
}

#[bench]
fn bench_line(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{line}"))
}

#[bench]
fn bench_column(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{column}"))
}

#[bench]
fn bench_module_path(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{module_path}"))
}

#[bench]
fn bench_logger(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{logger}"))
}

#[bench]
fn bench_payload(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{payload}"))
}

#[bench]
fn bench_pid(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{pid}"))
}

#[bench]
fn bench_tid(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{tid}"))
}

#[bench]
fn bench_eol(bencher: &mut Bencher) {
    bench_pattern(bencher, pattern!("{eol}"))
}
