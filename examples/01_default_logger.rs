use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

// All log macros and common types are already under `spdlog::prelude` module.
use spdlog::prelude::*;

fn main() {
    info!("program started");

    // Loggers only log 'info' level and more severe records by default,
    // you can modify the level filter of the default logger to enable all levels.
    let default_logger: Arc<Logger> = spdlog::default_logger();
    default_logger.set_level_filter(LevelFilter::All);

    trace!(
        "current unix timestamp: {:?}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Replacing the default logger.
    //
    // Suppose `new_logger` is the new default logger you want, we'll explain how to
    // build loggers in later examples.
    let new_logger: Arc<Logger> = spdlog::default_logger();
    let old_logger: Arc<Logger> = spdlog::swap_default_logger(new_logger);

    debug!(
        "the name of the old default logger: {:?}",
        old_logger.name()
    );

    info!("program exit");
}
