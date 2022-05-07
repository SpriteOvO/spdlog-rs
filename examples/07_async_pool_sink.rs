use std::{env, path::PathBuf, sync::Arc};

use spdlog::{
    prelude::*,
    sink::{AsyncPoolSink, FileSink},
};

fn main() {
    const LOG_FILE: &str = "logs/async_file_sink.log";

    let path: PathBuf = env::current_exe().unwrap().parent().unwrap().join(LOG_FILE);
    let file_sink: Arc<FileSink> = Arc::new(
        FileSink::builder()
            .path(&path)
            .truncate(true)
            .build()
            .unwrap(),
    );

    // Building a `AsyncPoolSink`.
    // Log and flush operations with this sink will be processed asynchronously.
    let async_pool_sink: Arc<AsyncPoolSink> =
        Arc::new(AsyncPoolSink::builder().sink(file_sink).build().unwrap());

    let logger: Arc<Logger> = Arc::new(Logger::builder().sink(async_pool_sink).build());

    info!(logger: logger, "hello async_pool_sink");
}
