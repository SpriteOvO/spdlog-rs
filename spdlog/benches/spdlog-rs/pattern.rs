#![feature(test)]

extern crate test;

use std::{cell::RefCell, sync::Arc};

use paste::paste;
#[cfg(feature = "serde_json")]
use spdlog::formatter::JsonFormatter;
use spdlog::{
    formatter::{pattern, Formatter, FullFormatter, Pattern, PatternFormatter},
    prelude::*,
    sink::Sink,
    Record, StringBuf,
};
use spdlog_macros::runtime_pattern;
use test::Bencher;

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

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

    fn level_filter(&self) -> LevelFilter {
        LevelFilter::All
    }

    fn set_level_filter(&self, _level_filter: LevelFilter) {
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
    let logger = build_test_logger(|b| b.sink(bench_sink));

    bencher.iter(|| info!(logger: logger, "payload"));
}

fn bench_pattern(bencher: &mut Bencher, pattern: impl Pattern + Clone + 'static) {
    bench_formatter(bencher, PatternFormatter::new(pattern));
}

fn bench_full_pattern(bencher: &mut Bencher, pattern: impl Pattern + Clone + 'static) {
    let full_formatter = Arc::new(StringSink::with(|b| {
        b.formatter(Box::new(FullFormatter::new()))
    }));

    let full_pattern = Arc::new(StringSink::with(|b| {
        b.formatter(Box::new(PatternFormatter::new(pattern.clone())))
    }));

    let combination =
        build_test_logger(|b| b.sink(full_formatter.clone()).sink(full_pattern.clone()));

    info!(logger: combination, "test payload");

    assert_eq!(full_formatter.clone_string(), full_pattern.clone_string());

    bench_pattern(bencher, pattern)
}

//

#[bench]
fn bench_1_full_formatter(bencher: &mut Bencher) {
    bench_formatter(bencher, FullFormatter::new())
}

#[cfg(feature = "serde_json")]
#[bench]
fn bench_1_json_formatter(bencher: &mut Bencher) {
    bench_formatter(bencher, JsonFormatter::new())
}

#[bench]
fn bench_2_full_pattern_ct(bencher: &mut Bencher) {
    bench_full_pattern(
        bencher,
        pattern!("[{date} {time}.{millisecond}] [{level}] {payload}{eol}"),
    )
}

#[bench]
fn bench_3_full_pattern_rt(bencher: &mut Bencher) {
    bench_full_pattern(
        bencher,
        runtime_pattern!("[{date} {time}.{millisecond}] [{level}] {payload}{eol}").unwrap(),
    )
}

macro_rules! bench_patterns {
    ( $(($name:ident, $placeholder:literal)),+ $(,)? ) => {
        $(paste! {
            #[bench]
            fn [<bench_4_ct_ $name>](bencher: &mut Bencher) {
                bench_pattern(bencher, pattern!($placeholder))
            }
            #[bench]
            fn [<bench_5_rt_ $name>](bencher: &mut Bencher) {
                bench_pattern(bencher, runtime_pattern!($placeholder).unwrap())
            }
        })+
    };
}

bench_patterns! {
    (weekday_name, "{weekday_name}"),
    (weekday_name_full, "{weekday_name_full}"),
    (month_name, "{month_name}"),
    (month_name_full, "{month_name_full}"),
    (datetime, "{datetime}"),
    (year_short, "{year_short}"),
    (year, "{year}"),
    (date_short, "{date_short}"),
    (date, "{date}"),
    (month, "{month}"),
    (day, "{day}"),
    (hour, "{hour}"),
    (hour_12, "{hour_12}"),
    (minute, "{minute}"),
    (second, "{second}"),
    (millsecond, "{millisecond}"),
    (microsecond, "{microsecond}"),
    (nanosecond, "{nanosecond}"),
    (am_pm, "{am_pm}"),
    (time_12, "{time_12}"),
    (time_short, "{time_short}"),
    (time, "{time}"),
    (tz_offset, "{tz_offset}"),
    (unix_timestamp, "{unix_timestamp}"),
    (full, "{full}"),
    (level, "{level}"),
    (level_short, "{level_short}"),
    (source, "{source}"),
    (file_name, "{file_name}"),
    (file, "{file}"),
    (line, "{line}"),
    (column, "{column}"),
    (module_path, "{module_path}"),
    (logger, "{logger}"),
    (payload, "{payload}"),
    (pid, "{pid}"),
    (tid, "{tid}"),
    (eol, "{eol}"),
}
