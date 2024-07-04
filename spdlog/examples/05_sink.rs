use std::sync::Arc;

use atomic::{Atomic, Ordering};
use spdlog::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    sink::Sink,
    ErrorHandler, Record, StringBuf,
};
use spin::{Mutex, RwLock};

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
        self.collected.lock().clone()
    }
}

impl Sink for CollectVecSink {
    fn log(&self, record: &Record) -> spdlog::Result<()> {
        let mut string_buf = StringBuf::new();
        self.formatter.read().format(record, &mut string_buf)?;
        self.collected.lock().push(string_buf.to_string());
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
        *self.formatter.write() = formatter;
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
    assert!(collected[0].ends_with(" [info] Hello, world!\n"));
    assert!(collected[1].ends_with(" [warn] Meow~\n"));

    Ok(())
}
