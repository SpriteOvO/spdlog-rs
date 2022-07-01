use std::{
    fmt::Write,
    ops::Range,
    sync::{Arc, Mutex},
};

use spdlog::{
    error,
    formatter::{Formatter, Pattern, PatternFormatter},
    pattern,
    sink::Sink,
    Logger, StringBuf,
};

#[test]
fn test_basic() {
    test_pattern(pattern!("hello"), "hello", None);
}

#[test]
fn test_builtin_formatters() {
    test_pattern(
        pattern!("{n}: [{level}] hello {v}"),
        "logger_name: [error] hello record_payload",
        None,
    );
}

#[test]
fn test_custom_formatters() {
    test_pattern(
        pattern!("{n}: [{level}] hello {v} - {mock1} / {mock2}",
            {"mock1"} => MockPattern1::default,
            {"mock2"} => MockPattern2::default,
        ),
        "logger_name: [error] hello record_payload - mock_pattern_1 / mock_pattern_2",
        None,
    );
}

#[test]
fn test_style_range() {
    test_pattern(
        pattern!("{n}: [{level}] {^hello$} {v}"),
        "logger_name: [error] hello record_payload",
        Some(21..26),
    );
}

fn test_pattern<P, F>(pat: P, expect_formatted: F, expect_style_range: Option<Range<usize>>)
where
    P: Pattern + 'static,
    F: AsRef<str>,
{
    let sink = MockSink::new();
    let formatter = PatternFormatter::new(pat);
    sink.set_formatter(Box::new(formatter));

    let sink = Arc::new(sink);
    let logger = Logger::builder()
        .name("logger_name")
        .sink(sink.clone())
        .build();
    error!(logger: logger, "record_payload");

    let (msg, style_range) = sink.get_last_msg().unwrap();
    assert_eq!(msg, expect_formatted.as_ref());
    assert_eq!(style_range, expect_style_range);
}

struct MockSink {
    formatter: Mutex<Option<Box<dyn Formatter>>>,
    last_msg: Mutex<Option<(String, Option<Range<usize>>)>>,
}

impl MockSink {
    fn new() -> Self {
        Self {
            formatter: Mutex::new(None),
            last_msg: Mutex::new(None),
        }
    }

    fn get_last_msg(&self) -> Option<(String, Option<Range<usize>>)> {
        self.last_msg.lock().unwrap().clone()
    }
}

impl Sink for MockSink {
    fn log(&self, record: &spdlog::Record) -> spdlog::Result<()> {
        let mut buf = StringBuf::new();
        let fmt = self.formatter.lock().unwrap();
        let extra_info = fmt.as_ref().unwrap().format(record, &mut buf).unwrap();
        *self.last_msg.lock().unwrap() =
            Some((String::from(buf.as_str()), extra_info.style_range()));
        Ok(())
    }

    fn flush(&self) -> spdlog::Result<()> {
        Ok(())
    }

    fn level_filter(&self) -> spdlog::LevelFilter {
        spdlog::LevelFilter::All
    }

    fn set_level_filter(&self, _level_filter: spdlog::LevelFilter) {}

    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        *self.formatter.lock().unwrap() = Some(formatter);
    }

    fn set_error_handler(&self, _handler: Option<spdlog::ErrorHandler>) {}

    fn should_log(&self, _level: spdlog::Level) -> bool {
        true
    }
}

#[derive(Default)]
struct MockPattern1;

impl Pattern for MockPattern1 {
    fn format(
        &self,
        _record: &spdlog::Record,
        dest: &mut StringBuf,
        _ctx: &mut spdlog::formatter::PatternContext,
    ) -> spdlog::Result<()> {
        write!(dest, "mock_pattern_1").unwrap();
        Ok(())
    }
}

#[derive(Default)]
struct MockPattern2;

impl Pattern for MockPattern2 {
    fn format(
        &self,
        _record: &spdlog::Record,
        dest: &mut StringBuf,
        _ctx: &mut spdlog::formatter::PatternContext,
    ) -> spdlog::Result<()> {
        write!(dest, "mock_pattern_2").unwrap();
        Ok(())
    }
}
