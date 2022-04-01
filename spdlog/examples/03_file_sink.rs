use std::{env, fs, path::PathBuf, sync::Arc, time::Duration};

use spdlog::{prelude::*, sink::FileSink};

fn main() {
    const LOG_FILE: &str = "logs/file_sink.log";

    let path: PathBuf = env::current_exe().unwrap().parent().unwrap().join(LOG_FILE);

    let file_sink: Arc<FileSink> = Arc::new(
        FileSink::builder()
            .path(&path)
            .truncate(true)
            .build()
            .unwrap(),
    );

    // Building a logger uses the `file_sink`.
    // All logs to this logger will be written to file "example_logs/file_sink.log".
    let logger: Arc<Logger> = Arc::new(Logger::builder().sink(file_sink).build());

    // Usually, if flush is relatively expensive for sinks, they do not
    // automatically flush on verbose levels by default, or even never,
    // depending on the sink implementation. However, they always do a final flush
    // when they are dropped.
    //
    // There are two automatic flush policies, and they can work together:
    //  - `Logger::set_flush_level_filter`
    //  - `Logger::set_flush_period`
    // For their description, see the documentation.

    // Flush when log "warn" and more severe logs.
    logger.set_flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));

    // Flush every 10 seconds.
    logger.set_flush_period(Some(Duration::from_secs(10)));

    info!(logger: logger, "hello");
    warn!(logger: logger, "world");

    drop(logger);

    println!(
        "contents of file '{}':\n{}",
        LOG_FILE,
        fs::read_to_string(path).unwrap()
    );
}
