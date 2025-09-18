// Test utils for unit tests, integration tests and doc tests
//
// In this file, you can only use public items from spdlog-rs, as this file will
// be used from integration tests and doc tests.
//
// This file will be handled in `build.rs` for code generation to workaround the
// rustdoc bug https://github.com/rust-lang/rust/issues/67295

use std::{
    fmt::Write,
    marker::PhantomData,
    sync::{atomic::*, Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use spdlog::{
    formatter::{Formatter, FormatterContext, Pattern, PatternFormatter},
    sink::{GetSinkProp, Sink, SinkProp, WriteSink, WriteSinkBuilder},
    Error, Logger, LoggerBuilder, Record, RecordOwned, Result, StringBuf,
};

//////////////////////////////////////////////////

pub struct TestSink {
    prop: SinkProp,
    log_counter: AtomicUsize,
    flush_counter: AtomicUsize,
    records: Mutex<Vec<RecordOwned>>,
    delay_duration: Option<Duration>,
}

impl TestSink {
    #[must_use]
    pub fn new() -> Self {
        Self::with_delay(None)
    }

    #[must_use]
    pub fn with_delay(duration: Option<Duration>) -> Self {
        Self {
            prop: SinkProp::default(),
            log_counter: AtomicUsize::new(0),
            flush_counter: AtomicUsize::new(0),
            records: Mutex::new(vec![]),
            delay_duration: duration,
        }
    }

    #[must_use]
    pub fn log_count(&self) -> usize {
        self.log_counter.load(Ordering::Relaxed)
    }

    #[must_use]
    pub fn flush_count(&self) -> usize {
        self.flush_counter.load(Ordering::Relaxed)
    }

    #[must_use]
    pub fn records(&self) -> Vec<RecordOwned> {
        self.records.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.records.lock().unwrap().clear();
    }

    #[must_use]
    pub fn payloads(&self) -> Vec<String> {
        self.records
            .lock()
            .unwrap()
            .iter()
            .map(|r| r.payload().to_string())
            .collect()
    }

    pub fn reset(&self) {
        self.log_counter.store(0, Ordering::Relaxed);
        self.flush_counter.store(0, Ordering::Relaxed);
        self.records.lock().unwrap().clear();
    }
}

impl GetSinkProp for TestSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for TestSink {
    fn log(&self, record: &Record) -> Result<()> {
        if let Some(delay) = self.delay_duration {
            sleep(delay);
        }

        self.log_counter.fetch_add(1, Ordering::Relaxed);
        self.records.lock().unwrap().push(record.to_owned());

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        if let Some(delay) = self.delay_duration {
            sleep(delay);
        }

        self.flush_counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Default for TestSink {
    fn default() -> Self {
        Self::new()
    }
}

//////////////////////////////////////////////////

pub struct StringSink {
    underlying: WriteSink<Vec<u8>>,
}

impl StringSink {
    pub fn new() -> Self {
        Self {
            underlying: WriteSink::builder().target(vec![]).build().unwrap(),
        }
    }

    pub fn with(
        cb: impl FnOnce(
            WriteSinkBuilder<Vec<u8>, PhantomData<Vec<u8>>>,
        ) -> WriteSinkBuilder<Vec<u8>, PhantomData<Vec<u8>>>,
    ) -> Self {
        Self {
            underlying: cb(WriteSink::builder().target(vec![])).build().unwrap(),
        }
    }

    pub fn clone_string(&self) -> String {
        String::from_utf8(self.underlying.clone_target()).unwrap()
    }
}

impl GetSinkProp for StringSink {
    fn prop(&self) -> &SinkProp {
        self.underlying.prop()
    }
}

impl Sink for StringSink {
    fn log(&self, record: &Record) -> Result<()> {
        self.underlying.log(record)
    }

    fn flush(&self) -> Result<()> {
        self.underlying.flush()
    }
}

//////////////////////////////////////////////////

// no modifications formatter, it will write `record` to `dest` as is.
#[derive(Clone)]
pub struct NoModFormatter {}

impl NoModFormatter {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Formatter for NoModFormatter {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut FormatterContext,
    ) -> Result<()> {
        dest.write_str(record.payload())
            .map_err(Error::FormatRecord)?;
        Ok(())
    }
}

impl Default for NoModFormatter {
    fn default() -> Self {
        Self::new()
    }
}

//////////////////////////////////////////////////

#[must_use]
pub fn build_test_logger(cb: impl FnOnce(&mut LoggerBuilder) -> &mut LoggerBuilder) -> Logger {
    let mut builder = Logger::builder();
    cb(builder.error_handler(|err| panic!("{}", err)));
    builder.build().unwrap()
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_trait {
    ($type:ty: $($traits:tt)+) => {{
        fn __assert_trait<T: $($traits)+>() {}
        __assert_trait::<$type>();
    }};
}
#[allow(unused_imports)]
pub use assert_trait;

#[must_use]
pub fn echo_logger_from_pattern(
    pattern: impl Pattern + Clone + 'static,
    name: Option<&'static str>,
) -> (Logger, Arc<StringSink>) {
    echo_logger_from_formatter(PatternFormatter::new(pattern), name)
}

#[must_use]
pub fn echo_logger_from_formatter(
    formatter: impl Formatter + 'static,
    name: Option<&'static str>,
) -> (Logger, Arc<StringSink>) {
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));

    let mut builder = Logger::builder();

    builder.sink(sink.clone());
    if let Some(name) = name {
        builder.name(name);
    }

    (builder.build().unwrap(), sink)
}
