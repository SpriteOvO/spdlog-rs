use std::{env, fmt::Write, fs, path::PathBuf, thread::sleep, time::Duration};

use crate::{
    formatter::{FmtExtraInfo, Formatter},
    sink::Sink,
    sync::*,
    Error, ErrorHandler, LevelFilter, LoggerBuilder, Record, Result, StringBuf,
};

pub static TEST_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("dev/test_logs");
    fs::create_dir_all(&path).unwrap();
    path
});

pub struct CounterSink {
    level_filter: Atomic<LevelFilter>,
    log_counter: AtomicUsize,
    flush_counter: AtomicUsize,
    payloads: Mutex<Vec<String>>,
    delay_duration: Option<Duration>,
}

// no modifications formatter, it will write `record` to `dest` as is.
#[derive(Clone)]
pub struct NoModFormatter {}

impl CounterSink {
    #[must_use]
    pub fn new() -> Self {
        Self::with_delay(None)
    }

    #[must_use]
    pub fn with_delay(duration: Option<Duration>) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            log_counter: AtomicUsize::new(0),
            flush_counter: AtomicUsize::new(0),
            payloads: Mutex::new(vec![]),
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
    pub fn payloads(&self) -> Vec<String> {
        self.payloads.lock_expect().clone()
    }

    pub fn reset(&self) {
        self.log_counter.store(0, Ordering::Relaxed);
        self.flush_counter.store(0, Ordering::Relaxed);
        self.payloads.lock_expect().clear();
    }
}

impl Sink for CounterSink {
    fn log(&self, record: &Record) -> Result<()> {
        if let Some(delay) = self.delay_duration {
            sleep(delay);
        }

        self.log_counter.fetch_add(1, Ordering::Relaxed);

        self.payloads
            .lock_expect()
            .push(record.payload().to_string());

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        if let Some(delay) = self.delay_duration {
            sleep(delay);
        }

        self.flush_counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn set_formatter(&self, _formatter: Box<dyn Formatter>) {
        // no-op
    }

    fn set_error_handler(&self, _handler: Option<ErrorHandler>) {
        // no-op
    }
}

impl Default for CounterSink {
    fn default() -> Self {
        Self::new()
    }
}

impl NoModFormatter {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Formatter for NoModFormatter {
    fn format(&self, record: &Record, dest: &mut StringBuf) -> Result<FmtExtraInfo> {
        dest.write_str(record.payload())
            .map_err(Error::FormatRecord)?;

        Ok(FmtExtraInfo::new())
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

impl Default for NoModFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn test_logger_builder() -> LoggerBuilder {
    let mut builder = LoggerBuilder::new();
    builder.error_handler(|err| panic!("{}", err));
    builder
}

pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}
