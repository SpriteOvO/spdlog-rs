use std::sync::{Arc, Mutex};

use spdlog::{
    formatter::FormatterContext,
    prelude::*,
    sink::{GetSinkProp, Sink, SinkProp},
    Record, StringBuf,
};

struct CollectVecSink {
    prop: SinkProp,
    collected: Mutex<Vec<String>>,
}

impl CollectVecSink {
    fn new() -> Self {
        Self {
            prop: SinkProp::default(),
            collected: Mutex::new(Vec::new()),
        }
    }

    fn collected(&self) -> Vec<String> {
        self.collected.lock().unwrap().clone()
    }
}

impl GetSinkProp for CollectVecSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for CollectVecSink {
    fn log(&self, record: &Record) -> spdlog::Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;
        self.collected.lock().unwrap().push(string_buf.to_string());
        Ok(())
    }

    fn flush(&self) -> spdlog::Result<()> {
        Ok(())
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
