//! A fast and flexible Rust logging library.
//!
//! Inspired by the C++ logging library [spdlog](https://github.com/gabime/spdlog).

#![warn(missing_docs)]

pub mod error;
pub mod formatter;
mod log_macros;
pub mod log_msg;
pub mod logger;
pub mod sink;
pub mod str_buf;
pub mod terminal;

pub use log::{Level, LevelFilter, Metadata, Record};

pub use error::{Error, ErrorHandler, Result};
pub use log_msg::LogMsg;
pub use str_buf::StrBuf;

use std::{fmt, sync::Arc};

use lazy_static::lazy_static;

use sink::StdoutStyleSink;

lazy_static! {
    static ref DEFAULT_LOGGER: Box<dyn logger::Logger> = Box::new(logger::BasicLogger::with_sink(
        Arc::new(StdoutStyleSink::default())
    ));
}

/// Initializes the crate
///
/// Users should initialize early at runtime and should only initialize once.
pub fn init() {
    lazy_static::initialize(&DEFAULT_LOGGER);
    log::set_max_level(log::LevelFilter::Info);
}

/// Returns a reference to the default logger.
pub fn default_logger() -> &'static dyn logger::Logger {
    DEFAULT_LOGGER.as_ref()
}

#[doc(hidden)]
pub fn __private_log(
    logger: &dyn logger::Logger,
    args: fmt::Arguments,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
) {
    logger.log(
        &Record::builder()
            .args(args)
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .build(),
    );
}
