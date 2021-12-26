use std::sync::Mutex;

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, Record, Result,
};

pub struct TestSink {
    level: LevelFilter,
    formatter: Box<dyn Formatter>,
    log_counter: Mutex<usize>,
    flush_counter: Mutex<usize>,
    payloads: Mutex<Vec<String>>,
}

impl TestSink {
    pub fn new() -> TestSink {
        TestSink {
            level: LevelFilter::Info,
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

    fn level(&self) -> LevelFilter {
        self.level
    }

    fn set_level(&mut self, level: LevelFilter) {
        self.level = level;
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
