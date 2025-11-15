use std::env;

use spdlog::{
    prelude::*,
    sink::{AsyncPoolSink, FileSink},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("async.log");
    let file_sink = FileSink::builder().path(path).build_arc()?;

    // AsyncPoolSink is a combined sink which wraps other sinks
    let async_pool_sink = AsyncPoolSink::builder().sink(file_sink).build_arc()?;

    let async_logger = Logger::builder()
        .sink(async_pool_sink)
        .flush_level_filter(LevelFilter::All)
        .build_arc()?;

    info!(logger: async_logger, "Hello, async!");

    Ok(())
}
