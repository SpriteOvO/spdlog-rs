use std::{env, fmt::Write, fs, path::PathBuf, sync::Mutex};

use lazy_static::lazy_static;

use crate::{
    formatter::{BasicFormatter, FmtExtraInfo, Formatter},
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

pub struct TestSink {
    level_filter: LevelFilter,
    formatter: Box<dyn Formatter>,
    log_counter: Mutex<usize>,
    flush_counter: Mutex<usize>,
    payloads: Mutex<Vec<String>>,
}

// no modifications formatter, it will write `record` to `dest` as is.
pub struct NoModFormatter {}

impl TestSink {
    pub fn new() -> TestSink {
        TestSink {
            level_filter: LevelFilter::All,
            formatter: Box::new(BasicFormatter::new()),
            log_counter: Mutex::new(0),
            flush_counter: Mutex::new(0),
            payloads: Mutex::new(vec![]),
        }
    }

    pub fn log_counter(&self) -> usize {
        *self.log_counter.lock().unwrap()
    }

    pub fn flush_counter(&self) -> usize {
        *self.flush_counter.lock().unwrap()
    }

    pub fn payloads(&self) -> Vec<String> {
        self.payloads.lock().unwrap().clone()
    }

    pub fn reset(&self) {
        *self.log_counter.lock().unwrap() = 0;
        *self.flush_counter.lock().unwrap() = 0;
        self.payloads.lock().unwrap().clear();
    }
}

impl Sink for TestSink {
    fn log(&self, record: &Record) -> Result<()> {
        *self.log_counter.lock().unwrap() += 1;

        self.payloads
            .lock()
            .unwrap()
            .push(record.payload().to_string());

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        *self.flush_counter.lock().unwrap() += 1;
        Ok(())
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter
    }

    fn set_level_filter(&mut self, level_filter: LevelFilter) {
        self.level_filter = level_filter;
    }

    fn formatter(&self) -> &dyn Formatter {
        self.formatter.as_ref()
    }

    fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
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

        Ok(FmtExtraInfo::default())
    }
}

pub fn test_logger_builder() -> LoggerBuilder {
    LoggerBuilder::new().error_handler(Box::new(|err| panic!("{}", err)))
}

pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}
