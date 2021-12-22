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
}

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error handler function type.
pub type ErrorHandler = Box<dyn FnMut(Error) + Send>;
