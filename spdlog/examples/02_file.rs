use std::{env, sync::Arc, time::Duration};

use spdlog::{
    prelude::*,
    sink::{FileSink, Sink},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `spdlog-rs` has a global default logger and logs will be processed by it
    // by default, You can configure it.
    let default_logger = spdlog::default_logger();
    default_logger.set_level_filter(LevelFilter::All);

    // Or completely replace it with a new one.
    let path = env::current_exe()?.with_file_name("all.log");
    let file_sink = Arc::new(FileSink::builder().path(path).build()?);

    let new_logger = Arc::new(
        Logger::builder()
            .level_filter(LevelFilter::All)
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
            .sink(file_sink.clone())
            .build()?,
    );
    new_logger.set_flush_period(Some(Duration::from_secs(3)));
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `all.log`");

    // In addition to having the global default logger, more loggers are allowed to
    // be configured, stored and used independently.
    let db = AppDatabase::new(file_sink)?;
    db.write_i32(114514);

    Ok(())
}

struct AppDatabase {
    logger: Logger,
}

impl AppDatabase {
    fn new(all_log_sink: Arc<dyn Sink>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = env::current_exe()?.with_file_name("db.log");
        let db_file_sink = Arc::new(FileSink::builder().path(path).build()?);

        let logger = Logger::builder()
            .name("database")
            .level_filter(LevelFilter::All)
            .flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn))
            .sinks([all_log_sink, db_file_sink])
            .build()?;
        Ok(Self { logger })
    }

    fn write_i32(&self, value: i32) {
        // This log will be written to both files `all.log` and `db.log`.
        trace!(logger: self.logger, "writing value {} to the database", value);
    }
}
