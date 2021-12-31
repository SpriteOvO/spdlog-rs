use std::sync::Mutex;

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, Record, Result,
};

pub struct TestSink {
    level_filter: LevelFilter,
    formatter: Box<dyn Formatter>,
    log_counter: Mutex<usize>,
    flush_counter: Mutex<usize>,
    payloads: Mutex<Vec<String>>,
}

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

pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}
