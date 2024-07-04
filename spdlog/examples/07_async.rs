use std::{env, sync::Arc};

use spdlog::{
    prelude::*,
    sink::{AsyncPoolSink, FileSink},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("async.log");
    let file_sink = Arc::new(FileSink::builder().path(path).build()?);

    // AsyncPoolSink is a combined sink which wraps other sinks
    let async_pool_sink = Arc::new(AsyncPoolSink::builder().sink(file_sink).build()?);

    let async_logger = Arc::new(
        Logger::builder()
            .sink(async_pool_sink)
            .flush_level_filter(LevelFilter::All)
            .build()?,
    );

    info!(logger: async_logger, "Hello, async!");

    Ok(())
}
