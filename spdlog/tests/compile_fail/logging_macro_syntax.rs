use spdlog::prelude::*;

fn optional_args() {
    let logger = spdlog::default_logger();

    log!(unknown: 1, Level::Info, "unknown optional arg");
    log!(Level::Info, logger: logger, "optional arg in the middle");
    log!(logger: logger, Level::Info, "duplicate optional args", logger: logger);
}

fn main() {}
