use std::{
    env,
    fmt::Write,
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use atomic::Atomic;
use lazy_static::lazy_static;

use crate::{
    formatter::{FmtExtraInfo, Formatter, FullFormatter},
    sink::Sink,
    Error, LevelFilter, LoggerBuilder, Record, Result, StringBuf,
};

lazy_static! {
    pub static ref TEST_LOGS_PATH: PathBuf = {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("dev/test_logs");
        fs::create_dir_all(&path).unwrap();
        path
    };
}

pub struct CounterSink {
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    log_counter: AtomicUsize,
    flush_counter: AtomicUsize,
    payloads: Mutex<Vec<String>>,
}

// no modifications formatter, it will write `record` to `dest` as is.
#[derive(Copy, Clone)]
pub struct NoModFormatter {}

impl CounterSink {
    pub fn new() -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            log_counter: AtomicUsize::new(0),
            flush_counter: AtomicUsize::new(0),
            payloads: Mutex::new(vec![]),
        }
    }

    pub fn log_count(&self) -> usize {
        self.log_counter.load(Ordering::Relaxed)
    }

    pub fn flush_count(&self) -> usize {
        self.flush_counter.load(Ordering::Relaxed)
    }

    pub fn payloads(&self) -> Vec<String> {
        self.payloads.lock().unwrap().clone()
    }

    pub fn reset(&self) {
        self.log_counter.store(0, Ordering::Relaxed);
        self.flush_counter.store(0, Ordering::Relaxed);
        self.payloads.lock().unwrap().clear();
    }
}

impl Sink for CounterSink {
    fn log(&self, record: &Record) -> Result<()> {
        self.log_counter.fetch_add(1, Ordering::Relaxed);

        self.payloads
            .lock()
            .unwrap()
            .push(record.payload().to_string());

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.flush_counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        *self.formatter.write() = formatter;
    }
}

impl Default for CounterSink {
    fn default() -> Self {
        Self::new()
    }
}

impl NoModFormatter {
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
}

impl Default for NoModFormatter {
    fn default() -> Self {
        Self::new()
    }
}

pub fn test_logger_builder() -> LoggerBuilder {
    LoggerBuilder::new().error_handler(|err| panic!("{}", err))
}

pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}
