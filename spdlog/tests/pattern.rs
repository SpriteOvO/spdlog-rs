use std::{
    fmt::Write,
    ops::{Range, RangeInclusive},
    sync::{Arc, Mutex},
};

use spdlog::{
    error,
    formatter::{pattern, Formatter, Pattern, PatternFormatter},
    prelude::*,
    sink::{Sink, WriteSink},
    StringBuf,
};

use cfg_if::cfg_if;
use regex::Regex;

#[test]
fn test_basic() {
    test_pattern(pattern!("hello"), "hello", None);
}

#[test]
fn test_builtin_formatters() {
    test_pattern(
        pattern!("{logger}: [{level}] hello {payload}"),
        "logger_name: [error] hello record_payload",
        None,
    );
}

#[test]
fn test_custom_formatters() {
    test_pattern(
        pattern!("{logger}: [{level}] hello {payload} - {$mock1} / {$mock2}",
            {$mock1} => MockPattern1::default,
            {$mock2} => MockPattern2::default,
        ),
        "logger_name: [error] hello record_payload - mock_pattern_1 / mock_pattern_2",
        None,
    );
}

#[test]
fn test_style_range() {
    test_pattern(
        pattern!("{logger}: [{level}] {^hello} {payload}"),
        "logger_name: [error] hello record_payload",
        Some(21..26),
    );
}

#[track_caller]
fn test_pattern<P, F>(pat: P, expect_formatted: F, expect_style_range: Option<Range<usize>>)
where
    P: Pattern + 'static + Clone,
    F: AsRef<str>,
{
    let sink = MockSink::new();
    let formatter = PatternFormatter::new(pat);
    sink.set_formatter(Box::new(formatter));

    let sink = Arc::new(sink);
    let logger = Logger::builder()
        .name("logger_name")
        .sink(sink.clone())
        .build()
        .unwrap();
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

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
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

#[test]
fn test_builtin_patterns() {
    #[track_caller]
    fn fmt(pattern: impl Pattern + Clone + 'static) -> String {
        let sink = Arc::new(
            WriteSink::builder()
                .formatter(Box::new(PatternFormatter::new(pattern)))
                .target(Vec::new())
                .build()
                .unwrap(),
        );

        let logger = Logger::builder()
            .sink(sink.clone())
            .name("logger-name")
            .build()
            .unwrap();

        info!(logger: logger, "test payload");

        String::from_utf8(sink.clone_target()).unwrap()
    }

    const WEEKDAY_NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const WEEKDAY_FULL_NAMES: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    const MONTH_NAMES: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const MONTH_FULL_NAMES: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    const AM_PM: [&str; 2] = ["AM", "PM"];

    #[track_caller]
    fn check(
        pattern: impl Pattern + Clone + 'static,
        template: Option<impl AsRef<str>>,
        ranges: Vec<RangeInclusive<u64>>,
    ) {
        #[track_caller]
        fn handle_datetime(input: impl Into<String>) -> (Vec<u64>, String) {
            let input = input.into();

            let digits = Regex::new(r"\d+")
                .unwrap()
                .captures_iter(&input)
                .map(|m| {
                    assert_eq!(m.len(), 1);
                    m.get(0).unwrap().as_str().parse::<u64>().unwrap()
                })
                .collect::<Vec<_>>();

            let mut res: String = input
                .chars()
                .map(|ch| if ch.is_ascii_digit() { '0' } else { ch })
                .collect();

            fn replace_substring(
                mut input: String,
                substring: impl IntoIterator<Item = &'static str>,
                target: &str,
            ) -> String {
                substring
                    .into_iter()
                    .for_each(|sub| input = input.replace(sub, target));
                input
            }

            res = replace_substring(res, WEEKDAY_FULL_NAMES, "{weekday_name_full}");
            res = replace_substring(res, MONTH_FULL_NAMES, "{month_name_full}");
            res = replace_substring(res, WEEKDAY_NAMES, "{weekday_name}");
            res = replace_substring(res, MONTH_NAMES, "{month_name}");
            res = replace_substring(res, AM_PM, "{am_pm}");

            #[cfg(not(windows))]
            const EOL: &str = "\n";
            #[cfg(windows)]
            const EOL: &str = "\r\n";
            res = replace_substring(res, [EOL], "{eol}");

            if res.starts_with('+') || res.starts_with('-') {
                res.replace_range(0..1, "{begin_sign}");
            }

            (digits, res)
        }

        let input = handle_datetime(fmt(pattern));

        println!(" => checking input '{input:?}'");

        assert_eq!(input.0.len(), ranges.len());

        input.0.iter().zip(ranges.iter()).for_each(|(d, r)| {
            assert!(r.contains(d));
        });

        if let Some(template) = template {
            assert_eq!(input.1, template.as_ref());
        }
    }

    const YEAR_RANGE: RangeInclusive<u64> = 2022..=9999;
    const YEAR_SHORT_RANGE: RangeInclusive<u64> = 0..=99;
    const MONTH_RANGE: RangeInclusive<u64> = 1..=12;
    const DAY_RANGE: RangeInclusive<u64> = 1..=31;
    const HOUR_RANGE: RangeInclusive<u64> = 0..=23;
    const HOUR_12_RANGE: RangeInclusive<u64> = 1..=12;
    const MINUTE_RANGE: RangeInclusive<u64> = 0..=59;
    const SECOND_RANGE: RangeInclusive<u64> = 0..=60; // `60` is considered leap second
    const MILLISECOND_RANGE: RangeInclusive<u64> = 0..=999;
    const MICROSECOND_RANGE: RangeInclusive<u64> = 0..=999999;
    const NANOSECOND_RANGE: RangeInclusive<u64> = 0..=999999999;
    #[cfg(feature = "source-location")]
    const SOURCE_RANGE: RangeInclusive<u64> = 0..=9999;
    const OS_ID_RANGE: RangeInclusive<u64> = 1..=u64::MAX;

    check(pattern!("{weekday_name}"), Some("{weekday_name}"), vec![]);
    check(
        pattern!("{weekday_name_full}"),
        Some("{weekday_name_full}"),
        vec![],
    );
    check(pattern!("{month_name}"), Some("{month_name}"), vec![]);
    check(
        pattern!("{month_name_full}"),
        Some("{month_name_full}"),
        vec![],
    );
    check(
        pattern!("{datetime}"),
        Some("{weekday_name} {month_name} 00 00:00:00 0000"),
        vec![
            DAY_RANGE,
            HOUR_RANGE,
            MINUTE_RANGE,
            SECOND_RANGE,
            YEAR_RANGE,
        ],
    );
    check(pattern!("{year_short}"), Some("00"), vec![YEAR_SHORT_RANGE]);
    check(pattern!("{year}"), Some("0000"), vec![YEAR_RANGE]);
    check(
        pattern!("{date_short}"),
        Some("00/00/00"),
        vec![MONTH_RANGE, DAY_RANGE, YEAR_SHORT_RANGE],
    );
    check(
        pattern!("{date}"),
        Some("0000-00-00"),
        vec![YEAR_RANGE, MONTH_RANGE, DAY_RANGE],
    );
    check(pattern!("{month}"), Some("00"), vec![MONTH_RANGE]);
    check(pattern!("{day}"), Some("00"), vec![DAY_RANGE]);
    check(pattern!("{hour}"), Some("00"), vec![HOUR_RANGE]);
    check(pattern!("{hour_12}"), Some("00"), vec![HOUR_12_RANGE]);
    check(pattern!("{minute}"), Some("00"), vec![MINUTE_RANGE]);
    check(pattern!("{second}"), Some("00"), vec![SECOND_RANGE]);
    check(
        pattern!("{millisecond}"),
        Some("000"),
        vec![MILLISECOND_RANGE],
    );
    check(
        pattern!("{microsecond}"),
        Some("000000"),
        vec![MICROSECOND_RANGE],
    );
    check(
        pattern!("{nanosecond}"),
        Some("000000000"),
        vec![NANOSECOND_RANGE],
    );
    check(pattern!("{am_pm}"), Some("{am_pm}"), vec![]);
    check(
        pattern!("{time_12}"),
        Some("00:00:00 {am_pm}"),
        vec![HOUR_12_RANGE, MINUTE_RANGE, SECOND_RANGE],
    );
    check(
        pattern!("{time_short}"),
        Some("00:00"),
        vec![HOUR_RANGE, MINUTE_RANGE],
    );
    check(
        pattern!("{time}"),
        Some("00:00:00"),
        vec![HOUR_RANGE, MINUTE_RANGE, SECOND_RANGE],
    );
    check(
        pattern!("{tz_offset}"),
        Some("{begin_sign}00:00"),
        vec![HOUR_RANGE, MINUTE_RANGE],
    );
    check(
        pattern!("{unix_timestamp}"),
        None as Option<&str>,
        vec![0..=i32::MAX as u64],
    );

    cfg_if! {
        if #[cfg(feature = "source-location")] {
            check(
                pattern!("{full}"),
                Some(format!("[0000-00-00 00:00:00.000] [logger-name] [info] [pattern, {}:000] test payload", file!())),
                vec![
                    YEAR_RANGE,
                    MONTH_RANGE,
                    DAY_RANGE,
                    HOUR_RANGE,
                    MINUTE_RANGE,
                    SECOND_RANGE,
                    MILLISECOND_RANGE,
                    SOURCE_RANGE,
                ],
            );
        } else {
            check(
                pattern!("{full}"),
                Some("[0000-00-00 00:00:00.000] [logger-name] [info] test payload"),
                vec![
                    YEAR_RANGE,
                    MONTH_RANGE,
                    DAY_RANGE,
                    HOUR_RANGE,
                    MINUTE_RANGE,
                    SECOND_RANGE,
                    MILLISECOND_RANGE,
                ],
            );
        }
    }

    check(pattern!("{level}"), Some("info"), vec![]);
    check(pattern!("{level_short}"), Some("I"), vec![]);
    cfg_if! {
        if #[cfg(feature = "source-location")] {
            check(pattern!("{source}"), Some(format!("{}:000", file!())), vec![SOURCE_RANGE]);
            check(pattern!("{file_name}"), Some("pattern.rs"), vec![]);
            check(pattern!("{file}"), Some(file!()), vec![]);
            check(pattern!("{line}"), Some("000"), vec![SOURCE_RANGE]);
            check(pattern!("{column}"), Some("0"), vec![SOURCE_RANGE]);
            check(pattern!("{module_path}"), Some(module_path!()), vec![]);
        } else {
            check(pattern!("{source}"), Some(""), vec![]);
            check(pattern!("{file_name}"), Some(""), vec![]);
            check(pattern!("{file}"), Some(""), vec![]);
            check(pattern!("{line}"), Some(""), vec![]);
            check(pattern!("{column}"), Some(""), vec![]);
            check(pattern!("{module_path}"), Some(""), vec![]);
        }
    }
    check(pattern!("{logger}"), Some("logger-name"), vec![]);
    check(pattern!("{payload}"), Some("test payload"), vec![]);
    check(pattern!("{pid}"), None as Option<&str>, vec![OS_ID_RANGE]);
    check(pattern!("{tid}"), None as Option<&str>, vec![OS_ID_RANGE]);
    check(pattern!("{eol}"), Some("{eol}"), vec![]);
}
