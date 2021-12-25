use std::sync::Mutex;

use crate::{logger::Logger, sink::Sinks, ErrorHandler, Level, LevelFilter, Record};

#[derive(Default)]
pub struct EmptyLogger {
    sinks: Sinks,
}

impl EmptyLogger {
    pub fn new() -> EmptyLogger {
        EmptyLogger::default()
    }
}

impl Logger for EmptyLogger {
    fn enabled(&self, _level: Level) -> bool {
        true
    }

    fn log(&self, _record: &Record) {}

    fn flush(&self) {}

    fn level(&self) -> LevelFilter {
        LevelFilter::Info
    }

    fn set_level(&mut self, _level: LevelFilter) {}

    fn sinks(&self) -> &Sinks {
        &self.sinks
    }

    fn sinks_mut(&mut self) -> &mut Sinks {
        self.sinks.as_mut()
    }

    fn sink_record(&self, _record: &Record) {}

    fn set_error_handler(&mut self, _handler: ErrorHandler) {}
}

#[derive(Default)]
pub struct TestLogger {
    sinks: Sinks,
    log_counter: Mutex<usize>,
    flush_counter: Mutex<usize>,
    payloads: Mutex<Vec<String>>,
}

impl TestLogger {
    pub fn new() -> TestLogger {
        TestLogger::default()
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

impl Logger for TestLogger {
    fn enabled(&self, _level: Level) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        *self.log_counter.lock().unwrap() += 1;

        self.sink_record(record);
    }

    fn flush(&self) {
        *self.flush_counter.lock().unwrap() += 1;
    }

    fn level(&self) -> LevelFilter {
        LevelFilter::Info
    }

    fn set_level(&mut self, _level: LevelFilter) {}

    fn sinks(&self) -> &Sinks {
        &self.sinks
    }

    fn sinks_mut(&mut self) -> &mut Sinks {
        self.sinks.as_mut()
    }

    fn sink_record(&self, record: &Record) {
        self.payloads
            .lock()
            .unwrap()
            .push(record.payload().to_string());
    }

    fn set_error_handler(&mut self, _handler: ErrorHandler) {}
}
