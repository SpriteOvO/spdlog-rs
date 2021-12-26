//! Provides error handling.

use std::{fmt, io, result};

use thiserror::Error;

/// The error type of this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// A formatting error.
    #[error("format error: {0}")]
    Format(#[from] fmt::Error),

    /// A IO error.
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// The type returned by [`from_str`] when the string doesn't match any of
    /// the log levels.
    ///
    /// [`from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
    #[error("attempted to convert a string that doesn't match an existing log level: {0}")]
    ParseLevel(String),
}

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error handler function type.
pub type ErrorHandler = Box<dyn Fn(Error) + Send + Sync>;
