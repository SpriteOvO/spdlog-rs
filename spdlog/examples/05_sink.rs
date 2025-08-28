use std::sync::{Arc, Mutex, RwLock};

use atomic::{Atomic, Ordering};
use spdlog::{
    formatter::{Formatter, FormatterContext, FullFormatter},
    prelude::*,
    sink::Sink,
    ErrorHandler, Record, StringBuf,
};

struct CollectVecSink {
    level_filter: Atomic<LevelFilter>,
    formatter: RwLock<Box<dyn Formatter>>,
    error_handler: Atomic<Option<ErrorHandler>>,
    collected: Mutex<Vec<String>>,
}

impl CollectVecSink {
    fn new() -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: RwLock::new(Box::new(FullFormatter::new())),
            error_handler: Atomic::new(None),
            collected: Mutex::new(Vec::new()),
        }
    }

    fn collected(&self) -> Vec<String> {
        self.collected.lock().unwrap().clone()
    }
}

impl Sink for CollectVecSink {
    fn log(&self, record: &Record) -> spdlog::Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.formatter
            .read()
            .unwrap()
            .format(record, &mut string_buf, &mut ctx)?;
        self.collected.lock().unwrap().push(string_buf.to_string());
        Ok(())
    }

    fn flush(&self) -> spdlog::Result<()> {
        Ok(())
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn set_formatter(&self, formatter: Box<dyn Formatter>) {
        *self.formatter.write().unwrap() = formatter;
    }

    fn set_error_handler(&self, handler: Option<ErrorHandler>) {
        self.error_handler.store(handler, Ordering::Relaxed);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_sink = Arc::new(CollectVecSink::new());
    let example = Logger::builder().sink(my_sink.clone()).build()?;

    info!(logger: example, "Hello, world!");
    warn!(logger: example, "Meow~");

    let collected = my_sink.collected();
    println!("collected:\n{collected:#?}");

    assert_eq!(collected.len(), 2);
    assert!(collected[0].contains("[info]") && collected[0].contains("Hello, world!"));
    assert!(collected[1].contains("[warn]") && collected[1].contains("Meow~"));

    Ok(())
}
