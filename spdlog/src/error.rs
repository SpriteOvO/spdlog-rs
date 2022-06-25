//! Provides error types.

use std::{fmt, io, result};

use atomic::Atomic;
use static_assertions::const_assert;
use thiserror::Error;

/// The error type of this crate.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The variant returned by [`Formatter`]s when an error occurs in
    /// formatting a record.
    ///
    /// [`Formatter`]: crate::formatter::Formatter
    #[error("format record error: {0}")]
    FormatRecord(fmt::Error),

    /// The variant returned by [`Sink`]s when an error occurs in writing a
    /// record to the target.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("write record error: {0}")]
    WriteRecord(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in flushing the
    /// buffer.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("flush buffer error: {0}")]
    FlushBuffer(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in creating a
    /// directory.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("create directory error: {0}")]
    CreateDirectory(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in opening a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("open file error: {0}")]
    OpenFile(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in querying the
    /// metadata of a file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("query file metadata error: {0}")]
    QueryFileMetadata(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in renaming a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("rename file error: {0}")]
    RenameFile(io::Error),

    /// The variant returned by [`Sink`]s when an error occurs in removing a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("remove file error: {0}")]
    RemoveFile(io::Error),

    /// The variant returned by [`from_str`] when the string doesn't match any
    /// of the log levels.
    ///
    /// [`from_str`]: std::str::FromStr::from_str
    #[error("attempted to convert a string that doesn't match an existing log level: {0}")]
    ParseLevel(String),

    /// The variant returned by [`Sink`]s when an error occurs in sending to the
    /// channel.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[cfg(feature = "multi-thread")]
    #[error("failed to send message to channel: {0}")]
    SendToChannel(SendToChannelError),
}

/// The more detailed error type of sending to channel.
#[cfg(feature = "multi-thread")]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SendToChannelError {
    /// The channel is full.
    ///
    /// The variant returned only when [`OverflowPolicy::DropIncoming`] is used.
    ///
    /// [`OverflowPolicy::DropIncoming`]: crate::sink::async_sink::OverflowPolicy::DropIncoming
    #[error("the channel is full")]
    Full,

    /// The channel is disconnected.
    #[error("the channel is disconnected")]
    Disconnected,
}

#[cfg(feature = "multi-thread")]
impl SendToChannelError {
    pub(crate) fn from_crossbeam<T>(err: crossbeam::channel::TrySendError<T>) -> Self {
        use crossbeam::channel::TrySendError;

        match err {
            TrySendError::Full(_) => Self::Full,
            TrySendError::Disconnected(_) => Self::Disconnected,
        }
    }
}

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error handler function type.
pub type ErrorHandler = fn(Error);

const_assert!(Atomic::<ErrorHandler>::is_lock_free());
const_assert!(Atomic::<Option<ErrorHandler>>::is_lock_free());
