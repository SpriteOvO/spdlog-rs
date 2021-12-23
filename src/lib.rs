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

#[doc(hidden)]
pub fn __private_log(
    args: fmt::Arguments,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
) {
    log::logger().log(
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
