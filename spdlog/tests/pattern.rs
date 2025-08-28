use std::{
    fmt::Write,
    ops::{Range, RangeInclusive},
    sync::{Arc, Mutex},
};

use cfg_if::cfg_if;
use regex::Regex;
#[cfg(feature = "runtime-pattern")]
use spdlog::formatter::runtime_pattern;
use spdlog::{
    error,
    formatter::{pattern, FormatterContext, Pattern, PatternFormatter},
    prelude::*,
    sink::{GetSinkProp, Sink, SinkProp},
    Error, StringBuf, __EOL,
};

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));

macro_rules! test_pattern {
    ( $template:literal, $($args: expr),+ $(,)? ) => {
        test_pattern_inner(pattern!($template), $($args),+);
        #[cfg(feature = "runtime-pattern")]
        test_pattern_inner(runtime_pattern!($template).unwrap(), $($args),+);
    };
    ( $patterns:expr, $($args: expr),+ $(,)? ) => {
        $patterns.into_iter().for_each(|pat| {
            test_pattern_inner(pat, $($args),+);
        });
    };
}

#[test]
fn test_basic() {
    test_pattern!("hello", "hello", None);
}

#[test]
fn test_builtin_formatters() {
    test_pattern!(
        "{logger}: [{level}] hello {payload}",
        "logger_name: [error] hello record_payload",
        None,
    );
}

#[test]
fn test_custom_formatters() {
    let mut patterns = vec![Box::new(
        pattern!("{logger}: [{level}] hello {payload} - {$mock1} / {$mock2}",
            {$mock1} => MockPattern1::default,
            {$mock2} => MockPattern2::default,
        ),
    ) as Box<dyn Pattern>];

    #[cfg(feature = "runtime-pattern")]
    patterns.push(Box::new(
        runtime_pattern!("{logger}: [{level}] hello {payload} - {$mock1} / {$mock2}",
            {$mock1} => MockPattern1::default,
            {$mock2} => MockPattern2::default,
        )
        .unwrap(),
    ));

    test_pattern!(
        patterns,
        "logger_name: [error] hello record_payload - mock_pattern_1 / mock_pattern_2",
        None,
    );
}

#[cfg(feature = "runtime-pattern")]
#[test]
fn test_unknown_custom_formatter() {
    let pattern = runtime_pattern!("{logger}: [{level}] hello {payload} - {$mock1} / {$mock2}",
        {$mock1} => MockPattern1::default,
    );
    assert!(pattern.is_err());
}

#[test]
fn test_style_range() {
    test_pattern!(
        "{logger}: [{level}] {^hello} {payload}",
        "logger_name: [error] hello record_payload",
        Some(21..26),
    );
}

#[track_caller]
fn test_pattern_inner<P, F>(pat: P, expect_formatted: F, expect_style_range: Option<Range<usize>>)
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
    prop: SinkProp,
    last_msg: Mutex<Option<(String, Option<Range<usize>>)>>,
}

impl MockSink {
    #[must_use]
    fn new() -> Self {
        Self {
            prop: SinkProp::default(),
            last_msg: Mutex::new(None),
        }
    }

    #[must_use]
    fn get_last_msg(&self) -> Option<(String, Option<Range<usize>>)> {
        self.last_msg.lock().unwrap().clone()
    }
}

impl GetSinkProp for MockSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for MockSink {
    fn log(&self, record: &spdlog::Record) -> spdlog::Result<()> {
        let mut buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut buf, &mut ctx)
            .unwrap();
        *self.last_msg.lock().unwrap() = Some((String::from(buf.as_str()), ctx.style_range()));
        Ok(())
    }

    fn flush(&self) -> spdlog::Result<()> {
        Ok(())
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
        let sink = Arc::new(test_utils::StringSink::with(|b| {
            b.formatter(Box::new(PatternFormatter::new(pattern)))
        }));

        let logger = Logger::builder()
            .sink(sink.clone())
            .name("logger-name")
            .build()
            .unwrap();

        info!(logger: logger, kv: { a=true, b="text" }, "test payload");

        sink.clone_string()
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
    // "May" is ambiguous for short or full :)
    const MONTH_NAMES: [&str; 11] = [
        "Jan", "Feb", "Mar", "Apr", // "May",
        "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    const MONTH_FULL_NAMES: [&str; 11] = [
        "January",
        "February",
        "March",
        "April",
        // "May",
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
    fn check_inner(
        pattern: impl Pattern + Clone + 'static,
        expected_templates: Option<impl IntoIterator<Item = impl AsRef<str>>>, // OR list
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
            res = replace_substring(res, ["May"], "{month_name|month_name_full}");
            res = replace_substring(res, AM_PM, "{am_pm}");
            res = replace_substring(res, [__EOL], "{eol}");

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

        if let Some(expected_templates) = expected_templates {
            assert!(expected_templates
                .into_iter()
                .any(|t| input.1 == t.as_ref()));
        }
    }

    macro_rules! check {
        ( $template:literal, $($args: expr),+ $(,)? ) => {
            check_inner(pattern!($template), $($args),+);
            #[cfg(feature = "runtime-pattern")]
            check_inner(runtime_pattern!($template).unwrap(), $($args),+);
        };
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

    check!("{weekday_name}", Some(["{weekday_name}"]), vec![]);
    check!("{weekday_name_full}", Some(["{weekday_name_full}"]), vec![]);
    check!(
        "{month_name}",
        Some(["{month_name}", "{month_name|month_name_full}"]),
        vec![]
    );
    check!(
        "{month_name_full}",
        Some(["{month_name_full}", "{month_name|month_name_full}"]),
        vec![]
    );
    check!(
        "{datetime}",
        Some([
            "{weekday_name} {month_name} 00 00:00:00 0000",
            "{weekday_name} {month_name|month_name_full} 00 00:00:00 0000"
        ]),
        vec![
            DAY_RANGE,
            HOUR_RANGE,
            MINUTE_RANGE,
            SECOND_RANGE,
            YEAR_RANGE,
        ],
    );
    check!("{year_short}", Some(["00"]), vec![YEAR_SHORT_RANGE]);
    check!("{year}", Some(["0000"]), vec![YEAR_RANGE]);
    check!(
        "{date_short}",
        Some(["00/00/00"]),
        vec![MONTH_RANGE, DAY_RANGE, YEAR_SHORT_RANGE],
    );
    check!(
        "{date}",
        Some(["0000-00-00"]),
        vec![YEAR_RANGE, MONTH_RANGE, DAY_RANGE],
    );
    check!("{month}", Some(["00"]), vec![MONTH_RANGE]);
    check!("{day}", Some(["00"]), vec![DAY_RANGE]);
    check!("{hour}", Some(["00"]), vec![HOUR_RANGE]);
    check!("{hour_12}", Some(["00"]), vec![HOUR_12_RANGE]);
    check!("{minute}", Some(["00"]), vec![MINUTE_RANGE]);
    check!("{second}", Some(["00"]), vec![SECOND_RANGE]);
    check!("{millisecond}", Some(["000"]), vec![MILLISECOND_RANGE]);
    check!("{microsecond}", Some(["000000"]), vec![MICROSECOND_RANGE]);
    check!("{nanosecond}", Some(["000000000"]), vec![NANOSECOND_RANGE]);
    check!("{am_pm}", Some(["{am_pm}"]), vec![]);
    check!(
        "{time_12}",
        Some(["00:00:00 {am_pm}"]),
        vec![HOUR_12_RANGE, MINUTE_RANGE, SECOND_RANGE],
    );
    check!(
        "{time_short}",
        Some(["00:00"]),
        vec![HOUR_RANGE, MINUTE_RANGE],
    );
    check!(
        "{time}",
        Some(["00:00:00"]),
        vec![HOUR_RANGE, MINUTE_RANGE, SECOND_RANGE],
    );
    check!(
        "{tz_offset}",
        Some(["{begin_sign}00:00"]),
        vec![HOUR_RANGE, MINUTE_RANGE],
    );
    check!(
        "{unix_timestamp}",
        None as Option<Vec<&str>>,
        vec![0..=i32::MAX as u64],
    );

    cfg_if! {
        if #[cfg(feature = "source-location")] {
            check!(
                "{full}",
                Some([format!("[0000-00-00 00:00:00.000] [logger-name] [info] [pattern, {}:000] test payload {{ a=true b=text }}", file!())]),
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
            check!(
                "{full}",
                Some(["[0000-00-00 00:00:00.000] [logger-name] [info] test payload { a=true b=text }"]),
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

    check!("{level}", Some(["info"]), vec![]);
    check!("{level_short}", Some(["I"]), vec![]);
    cfg_if! {
        if #[cfg(feature = "source-location")] {
            check!("{source}", Some([format!("{}:000", file!())]), vec![SOURCE_RANGE]);
            check!("{file_name}", Some(["pattern.rs"]), vec![]);
            check!("{file}", Some([file!()]), vec![]);
            check!("{line}", Some(["000"]), vec![SOURCE_RANGE]);
            check!("{column}", Some(["0"]), vec![SOURCE_RANGE]);
            check!("{module_path}", Some([module_path!()]), vec![]);
        } else {
            check!("{source}", Some([""]), vec![]);
            check!("{file_name}", Some([""]), vec![]);
            check!("{file}", Some([""]), vec![]);
            check!("{line}", Some([""]), vec![]);
            check!("{column}", Some([""]), vec![]);
            check!("{module_path}", Some([""]), vec![]);
        }
    }
    check!("{logger}", Some(["logger-name"]), vec![]);
    check!("{payload}", Some(["test payload"]), vec![]);
    check!("{kv}", Some(["a=true b=text"]), vec![]);
    check!("{pid}", None as Option<Vec<&str>>, vec![OS_ID_RANGE]);
    check!("{tid}", None as Option<Vec<&str>>, vec![OS_ID_RANGE]);
    check!("{eol}", Some(["{eol}"]), vec![]);
}

#[cfg(feature = "runtime-pattern")]
fn custom_pat_creator() -> impl Pattern {
    spdlog::formatter::__pattern::Level
}

#[cfg(feature = "runtime-pattern")]
#[test]
fn runtime_pattern_valid() {
    assert!(runtime_pattern!("").is_ok());
    assert!(runtime_pattern!("{logger}").is_ok());
    assert!(
        runtime_pattern!("{logger} {$custom_pat}", {$custom_pat} => custom_pat_creator).is_ok()
    );
    assert!(
        runtime_pattern!("{logger} {$_custom_pat}", {$_custom_pat} => custom_pat_creator).is_ok()
    );
    assert!(
        runtime_pattern!("{logger} {$_2custom_pat}", {$_2custom_pat} => custom_pat_creator).is_ok()
    );
}

#[cfg(feature = "runtime-pattern")]
#[test]
fn runtime_pattern_invalid() {
    assert!(matches!(
        runtime_pattern!("{logger-name}"),
        Err(Error::BuildPattern(_))
    ));
    assert!(matches!(
        runtime_pattern!("{nonexistent}"),
        Err(Error::BuildPattern(_))
    ));
    assert!(matches!(
        runtime_pattern!("{}"),
        Err(Error::BuildPattern(_))
    ));
    assert!(matches!(
        runtime_pattern!("{logger} {$custom_pat_no_ref}"),
        Err(Error::BuildPattern(_))
    ));
    assert!(matches!(
        runtime_pattern!("{logger} {$custom_pat}", {$r#custom_pat} => custom_pat_creator),
        Err(Error::BuildPattern(_))
    ));
    assert!(matches!(
        runtime_pattern!("{logger} {$r#custom_pat}", {$r#custom_pat} => custom_pat_creator),
        Err(Error::BuildPattern(_))
    ));
}

#[cfg(feature = "multi-thread")]
#[test]
fn test_different_context_thread() {
    use std::time::Duration;

    use spdlog::{sink::AsyncPoolSink, ThreadPool};

    let formatter = Box::new(PatternFormatter::new(pattern!("{tid}{eol}")));
    let thread_pool = Arc::new(ThreadPool::builder().build().unwrap());
    let buffer_sink = Arc::new(test_utils::StringSink::with(|b| b.formatter(formatter)));
    let sinks: [Arc<dyn Sink>; 2] = [
        buffer_sink.clone(),
        Arc::new(
            AsyncPoolSink::builder()
                .sink(buffer_sink.clone())
                .thread_pool(thread_pool)
                .build()
                .unwrap(),
        ),
    ];
    let logger = Arc::new(Logger::builder().sinks(sinks).build().unwrap());

    info!(logger: logger, "");
    std::thread::sleep(Duration::from_millis(200));

    let buffer = buffer_sink.clone_string();
    let buffer = buffer.lines().collect::<Vec<_>>();
    assert_eq!(buffer.len(), 2);
    buffer.windows(2).for_each(|w| assert_eq!(w[0], w[1]))
}
