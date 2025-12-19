//! Provides error types.
//!
//! # Default error handler
//!
//! If a logger or sink does not have an error handler set up, a default error
//! handler will be used, which will print the error to `stderr`.

use std::{
    error::Error as StdError,
    fmt::{self, Display},
    io::{self, Write as _},
    result,
    sync::Arc,
};

use thiserror::Error;

pub use crate::env_level::EnvLevelError;
#[cfg(feature = "multi-thread")]
use crate::{sink::Task, RecordOwned};

/// Contains most errors of this crate.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Returned by [`Formatter`]s when an error occurs in formatting a record.
    ///
    /// [`Formatter`]: crate::formatter::Formatter
    #[error("format record error: {0}")]
    FormatRecord(fmt::Error),

    /// Returned by [`Sink`]s when an error occurs in writing a record to the
    /// target.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("write record error: {0}")]
    WriteRecord(io::Error),

    /// Returned by [`Sink`]s when an error occurs in flushing the buffer.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("flush buffer error: {0}")]
    FlushBuffer(io::Error),

    /// Returned by [`Sink`]s when an error occurs in creating a directory.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("create directory error: {0}")]
    CreateDirectory(io::Error),

    /// Returned by [`Sink`]s when an error occurs in opening a file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("open file error: {0}")]
    OpenFile(io::Error),

    /// Returned by [`Sink`]s when an error occurs in querying the metadata of a
    /// file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("query file metadata error: {0}")]
    QueryFileMetadata(io::Error),

    /// Returned by [`Sink`]s when an error occurs in renaming a file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("rename file error: {0}")]
    RenameFile(io::Error),

    /// Returned by [`Sink`]s when an error occurs in removing a file.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[error("remove file error: {0}")]
    RemoveFile(io::Error),

    /// Returned by [`from_str`] when the string doesn't match any of the log
    /// levels.
    ///
    /// [`from_str`]: std::str::FromStr::from_str
    #[error("attempted to convert a string that doesn't match an existing log level: {0}")]
    ParseLevel(String),

    /// Returned if an invalid argument was passed in.
    #[error("invalid argument {0}")]
    InvalidArgument(#[from] InvalidArgumentError),

    /// Returned by [`Sink`]s when an error occurs in sending to the channel.
    ///
    /// [`Sink`]: crate::sink::Sink
    #[cfg(feature = "multi-thread")]
    #[error("failed to send message to channel: {0}")]
    SendToChannel(SendToChannelError, SendToChannelErrorDropped),

    /// Returned by [`runtime_pattern!`] when the pattern is failed to be built
    /// at runtime.
    ///
    /// [`runtime_pattern!`]: crate::formatter::runtime_pattern
    #[cfg(feature = "runtime-pattern")]
    #[error("failed to build pattern at runtime: {0}")]
    BuildPattern(BuildPatternError),

    /// Returned by [`Formatter`]s when an error occurs in serializing a log.
    ///
    /// [`Formatter`]: crate::formatter::Formatter
    #[cfg(feature = "serde")]
    #[error("failed to serialize log: {0}")]
    SerializeRecord(io::Error),

    /// Returned from a downstream implementation of `spdlog-rs`. Its actual
    /// error type may be a downstream struct.
    ///
    /// When downstream crates encounter errors, other more specific error
    /// variants should be used first, this variant should only be used as a
    /// last option when other variant types are incompatible.
    #[error("{0}")]
    Downstream(Box<dyn StdError + Send + Sync>),

    /// Returned when multiple errors occurred.
    #[error("{0:?}")]
    Multiple(Vec<Error>),

    #[cfg(test)]
    #[doc(hidden)]
    #[error("{0}")]
    __ForInternalTestsUseOnly(i32),
}

/// Indicates that an invalid parameter was specified.
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
    #[deprecated(
        since = "0.5.0",
        note = "non-zero thread pool capacity is now guarded by NonZeroUsize type"
    )]
    #[error("'thread pool capacity': {0}")]
    ThreadPoolCapacity(String),
}

/// Indicates that an invalid logger name was set.
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

/// Indicates that an error occurred while sending to channel.
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
    Record(Box<RecordOwned>), // Boxed because `RecordOwned` is a bit large.
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
            Task::Log { record, .. } => Self::Record(Box::new(record)),
            Task::Flush { .. } => Self::Flush,
        }
    }
}

/// Indicates that an error occurred while building a pattern at compile-time.
#[cfg(feature = "runtime-pattern")]
#[derive(Error, Debug)]
#[error("{0}")]
pub struct BuildPatternError(pub(crate) spdlog_internal::pattern_parser::Error);

/// The result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// Represents an error handler.
///
/// In most cases, it can be constructed by just a `.into()`.
///
/// Call [`ErrorHandler::default`] to construct an empty error handler, when an
/// error is triggered, a built-in fallback handler will be used which prints
/// the error to `stderr`.
#[derive(Clone)]
pub struct ErrorHandler(Option<Arc<dyn Fn(Error) + Send + Sync>>);

impl ErrorHandler {
    /// Constructs an error handler with a custom function.
    #[must_use]
    pub fn new<F>(custom: F) -> Self
    where
        F: Fn(Error) + Send + Sync + 'static,
    {
        Self(Some(Arc::new(custom)))
    }

    /// Calls the error handler with an error.
    pub fn call(&self, err: Error) {
        self.call_internal("External", err);
    }

    pub(crate) fn call_internal(&self, from: impl AsRef<str>, err: Error) {
        if let Some(handler) = &self.0 {
            handler(err);
        } else {
            Self::default_impl(from, err);
        }
    }

    fn default_impl(from: impl AsRef<str>, error: Error) {
        if let Error::Multiple(errs) = error {
            errs.into_iter()
                .for_each(|err| Self::default_impl(from.as_ref(), err));
            return;
        }

        let date = chrono::Local::now()
            .format("%Y-%m-%d %H:%M:%S.%3f")
            .to_string();

        // https://github.com/SpriteOvO/spdlog-rs/discussions/87
        //
        // Don't use `eprintln!` here, as it may fail to write and then panic.
        let _ = writeln!(
            io::stderr(),
            "[*** SPDLOG-RS UNHANDLED ERROR ***] [{}] [{}] {}",
            date,
            from.as_ref(),
            error
        );
    }
}

impl<F> From<F> for ErrorHandler
where
    F: Fn(Error) + Send + Sync + 'static,
{
    fn from(handler: F) -> Self {
        Self::new(handler)
    }
}

impl Default for ErrorHandler {
    /// Constructs an error handler with the built-in handler which prints
    /// errors to `stderr`.
    fn default() -> Self {
        Self(None)
    }
}

impl fmt::Debug for ErrorHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ErrorHandler")
            .field(&self.0.as_ref().map_or("default", |_| "custom"))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn error_traits() {
        assert_trait!(Error: Send + Sync);
    }

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
