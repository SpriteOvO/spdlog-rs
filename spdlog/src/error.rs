//! Provides error types.

use std::{
    fmt::{self, Display},
    io, result,
};

use atomic::Atomic;
use thiserror::Error;

pub use crate::env_level::EnvLevelError;
use crate::utils::const_assert;
#[cfg(feature = "multi-thread")]
use crate::{sink::Task, RecordOwned};

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

    /// The variant returned if an invalid argument was passed in.
    #[error("invalid argument {0}")]
    InvalidArgument(#[from] InvalidArgumentError),

    /// The variant returned by [`Sink`]s when an error occurs in sending to the
    /// channel.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[cfg(feature = "multi-thread")]
    #[error("failed to send message to channel: {0}")]
    SendToChannel(SendToChannelError, SendToChannelErrorDropped),

    /// The variant returned by [`RuntimePatternBuilder::build`] when the
    /// pattern is failed to be built at runtime.
    ///
    /// [`RuntimePatternBuilder::build`]: crate::formatter::RuntimePatternBuilder::build
    #[cfg(feature = "runtime-pattern")]
    #[error("failed to build pattern at runtime: {0}")]
    BuildPattern(BuildPatternError),

    /// This variant returned when multiple errors occurred.
    #[error("{0:?}")]
    Multiple(Vec<Error>),

    #[cfg(test)]
    #[error("{0}")]
    __ForInternalTestsUseOnly(i32),
}

/// This error type contains a variety of possible invalid arguments.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum InvalidArgumentError {
    /// Invalid logger name.
    ///
    /// See the documentation of [`LoggerBuilder::name`] for the name
    /// requirements.
    ///
    /// [`LoggerBuilder::name`]: crate::LoggerBuilder::name
    #[error("'logger name': {0}")]
    LoggerName(#[from] SetLoggerNameError),

    /// Invalid [`RotationPolicy`].
    ///
    /// See the documentation of [`RotationPolicy`] for the input requirements.
    ///
    /// [`RotationPolicy`]: crate::sink::RotationPolicy
    #[error("'rotation policy': {0}")]
    RotationPolicy(String),

    /// Invalid thread pool capacity.
    #[error("'thread pool capacity': {0}")]
    ThreadPoolCapacity(String),
}

#[cfg(feature = "runtime-pattern")]
impl Error {
    pub(crate) fn err_build_pattern(err: BuildPatternErrorInner) -> Self {
        Self::BuildPattern(BuildPatternError(err))
    }
    pub(crate) fn err_build_pattern_internal(err: spdlog_internal::pattern_parser::Error) -> Self {
        Self::BuildPattern(BuildPatternError(BuildPatternErrorInner::Internal(err)))
    }
}

/// This error indicates that an invalid logger name was set.
///
/// See the documentation of [`LoggerBuilder::name`] for the name requirements.
///
/// [`LoggerBuilder::name`]: crate::LoggerBuilder::name
#[derive(Error, Debug)]
pub struct SetLoggerNameError {
    name: String,
}

impl SetLoggerNameError {
    #[must_use]
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

impl Display for SetLoggerNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name '{}' contains disallowed characters", self.name)
    }
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

/// Contains data that is dropped after sending to the channel failed.
///
/// You can handle them manually or just ignore them.
#[cfg(feature = "multi-thread")]
#[derive(Debug)]
#[non_exhaustive]
pub enum SendToChannelErrorDropped {
    /// A `log` operation and a record are dropped.
    // TODO: Box the `RecordOwned` in the next minor version, as it's a bit large.
    Record(RecordOwned),
    /// A `flush` operation is dropped.
    Flush,
}

impl Error {
    pub(crate) fn push_err<T>(result: Result<T>, new: Self) -> Result<T> {
        match result {
            Ok(_) => Err(new),
            Err(Self::Multiple(mut errors)) => {
                errors.push(new);
                Err(Self::Multiple(errors))
            }
            Err(prev) => Err(Error::Multiple(vec![prev, new])),
        }
    }

    pub(crate) fn push_result<T, N>(result: Result<T>, new: Result<N>) -> Result<T> {
        match new {
            Ok(_) => result,
            Err(err) => Self::push_err(result, err),
        }
    }
}

#[cfg(feature = "multi-thread")]
impl Error {
    #[must_use]
    pub(crate) fn from_crossbeam_send(err: crossbeam::channel::SendError<Task>) -> Self {
        Self::SendToChannel(
            SendToChannelError::Disconnected,
            SendToChannelErrorDropped::from_task(err.0),
        )
    }

    #[must_use]
    pub(crate) fn from_crossbeam_try_send(err: crossbeam::channel::TrySendError<Task>) -> Self {
        use crossbeam::channel::TrySendError;

        let (error, dropped_task) = match err {
            TrySendError::Full(dropped) => (SendToChannelError::Full, dropped),
            TrySendError::Disconnected(dropped) => (SendToChannelError::Disconnected, dropped),
        };

        Self::SendToChannel(error, SendToChannelErrorDropped::from_task(dropped_task))
    }
}

#[cfg(feature = "multi-thread")]
impl SendToChannelErrorDropped {
    #[must_use]
    pub(crate) fn from_task(task: Task) -> Self {
        match task {
            Task::Log { record, .. } => Self::Record(record),
            Task::Flush { .. } => Self::Flush,
        }
    }
}

/// This error indicates that an error occurred while building a pattern at
/// compile-time.
#[cfg(feature = "runtime-pattern")]
#[derive(Error, Debug)]
#[error("{0}")]
pub struct BuildPatternError(BuildPatternErrorInner);

#[cfg(feature = "runtime-pattern")]
#[derive(Error, Debug)]
pub(crate) enum BuildPatternErrorInner {
    #[error("{0}")]
    Internal(spdlog_internal::pattern_parser::Error),
    #[error("invalid placeholder for custom pattern '{0}'")]
    InvalidCustomPlaceholder(String),
}

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// The error handler function type.
pub type ErrorHandler = fn(Error);

const_assert!(Atomic::<ErrorHandler>::is_lock_free());
const_assert!(Atomic::<Option<ErrorHandler>>::is_lock_free());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_err() {
        macro_rules! make_err {
            ( $($inputs:tt)+ ) => {
                Error::__ForInternalTestsUseOnly($($inputs)*)
            };
        }

        assert!(matches!(
            Error::push_err(Ok(()), make_err!(1)),
            Err(make_err!(1))
        ));

        assert!(matches!(
            Error::push_err::<()>(Err(make_err!(1)), make_err!(2)),
            Err(Error::Multiple(v)) if matches!(v[..], [make_err!(1), make_err!(2)])
        ));
    }
}
