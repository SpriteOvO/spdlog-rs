// All log macros and common types are under `spdlog::prelude` module.
use spdlog::prelude::*;

fn main() {
    // Writes a log at "info" level with the info level, and this log will be
    // processed by the global default logger - It will be output to `stdout`.
    info!("program started");

    let file = "config.json";

    // They will be output to `stderr`.
    error!("failed to open file: {}", file);
    warn!("undetermined locale, defaults to `en_US.UTF-8`");

    // Level "trace" and "debug" will be ignored by default, you can modify the
    // level filter of the global default logger to enable all levels.
    spdlog::default_logger().set_level_filter(LevelFilter::All);

    trace!("position x: {}, y: {}", 11.4, -5.14);
}
