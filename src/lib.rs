//! A fast and flexible Rust logging library.
//!
//! Inspired by the C++ logging library [spdlog](https://github.com/gabime/spdlog).

#![warn(missing_docs)]

pub mod error;
pub mod formatter;
pub mod log_msg;
pub mod logger;
pub mod sink;
pub mod str_buf;
pub mod terminal;

pub use log::{debug, error, info, log, trace, warn, Level, LevelFilter, Metadata, Record};

pub use error::{Error, ErrorHandler, Result};
pub use log_msg::LogMsg;
pub use str_buf::StrBuf;

use std::sync::Arc;

use sink::StdoutStyleSink;

/// Initializes the crate
///
/// Users should initialize early at runtime and should only initialize once.
/// Any log messages generated before the crate is initialized will be ignored.
pub fn init() {
    log::set_boxed_logger(Box::new(logger::BasicLogger::with_sink(Arc::new(
        StdoutStyleSink::default(),
    ))))
    .map(|()| log::set_max_level(log::LevelFilter::Info))
    .unwrap()
}
