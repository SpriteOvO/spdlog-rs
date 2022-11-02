//! Provides a log record structure.

use std::{
    borrow::{Borrow, Cow},
    time::SystemTime,
};

use crate::{Level, SourceLocation};

/// Represents a log record.
///
/// # Use
///
/// `Record` structures are passed as arguments to methods [`Logger::log`].
/// Loggers forward these structures to its sinks, then sink implementors
/// manipulate these structures in order to process log records. `Record`s are
/// automatically created by log macros and so are not seen by log users.
///
/// [`Logger::log`]: crate::logger::Logger::log
/// [`Sink::log`]: crate::sink::Sink::log
/// [`log!`]: crate::log
//
// FIXME: `Record` still owns some data and not just a reference, I'm not sure this is necessary and
// possible to correct.
#[derive(Clone, Debug)]
pub struct Record<'a> {
    logger_name: Option<&'a str>,
    payload: Cow<'a, str>,
    inner: Cow<'a, RecordInner>,
}

#[derive(Clone, Debug)]
struct RecordInner {
    level: Level,
    source_location: Option<SourceLocation>,
    time: SystemTime,
}

impl<'a> Record<'a> {
    /// Constructs a `Record`.
    ///
    /// [`Sink`]: crate::sink::Sink
    pub(crate) fn new<S>(level: Level, payload: S) -> Record<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Record {
            logger_name: None,
            payload: payload.into(),
            inner: Cow::Owned(RecordInner {
                level,
                source_location: None,
                time: SystemTime::now(),
            }),
        }
    }

    /// Constructs a [`RecordBuilder`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub(crate) fn builder<S>(level: Level, payload: S) -> RecordBuilder<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        RecordBuilder::new(level, payload)
    }

    /// Creates a [`RecordOwned`] that doesn't have lifetimes.
    pub fn to_owned(&self) -> RecordOwned {
        RecordOwned {
            logger_name: self.logger_name.map(|n| n.into()),
            payload: self.payload.to_string(),
            inner: self.inner.clone().into_owned(),
        }
    }

    /// Gets the logger name.
    pub fn logger_name(&self) -> Option<&'a str> {
        self.logger_name
    }

    /// Gets the level.
    pub fn level(&self) -> Level {
        self.inner.level
    }

    /// Gets the payload.
    pub fn payload(&self) -> &str {
        self.payload.borrow()
    }

    /// Gets the source location.
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.inner.source_location.as_ref()
    }

    /// Gets the time when the record was created.
    pub fn time(&self) -> SystemTime {
        self.inner.time
    }

    // When adding more getters, also add to `RecordOwned`

    #[cfg(feature = "log")]
    pub(crate) fn from_log_crate_record(
        logger: &'a crate::Logger,
        record: &log::Record,
        time: SystemTime,
    ) -> Self {
        let args = record.args();

        Self {
            logger_name: logger.name(),
            payload: match args.as_str() {
                Some(literal_str) => literal_str.into(),
                None => args.to_string().into(),
            },
            inner: Cow::Owned(RecordInner {
                level: record.level().into(),
                // `module_path` and `file` in `log::Record` are not `'static`
                source_location: None,
                time,
            }),
        }
    }

    #[cfg(test)]
    pub(crate) fn set_time(&mut self, new: SystemTime) {
        self.inner.to_mut().time = new;
    }
}

/// [`Record`] without lifetimes version.
// We do not `impl From<&Record> for RecordOwned` because it does not follow the
// Rust naming convention. Use `record.to_owned()` instead.
#[derive(Clone, Debug)]
pub struct RecordOwned {
    logger_name: Option<String>,
    payload: String,
    inner: RecordInner,
}

impl RecordOwned {
    // For internal (benchmark) use only.
    #[doc(hidden)]
    pub const __SIZE_OF: usize = std::mem::size_of::<Self>();

    /// References as [`Record`] cheaply.
    pub fn as_ref(&self) -> Record {
        Record {
            logger_name: self.logger_name.as_deref(),
            payload: Cow::Borrowed(&self.payload),
            inner: Cow::Borrowed(&self.inner),
        }
    }

    /// Gets the logger name.
    pub fn logger_name(&self) -> Option<&str> {
        self.logger_name.as_deref()
    }

    /// Gets the level.
    pub fn level(&self) -> Level {
        self.inner.level
    }

    /// Gets the payload.
    pub fn payload(&self) -> &str {
        self.payload.borrow()
    }

    /// Gets the source location.
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.inner.source_location.as_ref()
    }

    /// Gets the time when the record was created.
    pub fn time(&self) -> SystemTime {
        self.inner.time
    }

    // When adding more getters, also add to `Record`
}

/// The builder of [`Record`].
///
/// [`Sink`]: crate::sink::Sink
#[derive(Clone, Debug)]
pub(crate) struct RecordBuilder<'a> {
    record: Record<'a>,
}

impl<'a> RecordBuilder<'a> {
    /// Constructs a `RecordBuilder`.
    ///
    /// The default value of [`Record`] is the same as [`Record::new()`].
    ///
    /// Typically users should only use it for testing [`Sink`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub(crate) fn new<S>(level: Level, payload: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            record: Record::new(level, payload),
        }
    }

    /// Sets the logger name.
    #[must_use]
    pub(crate) fn logger_name(mut self, logger_name: &'a str) -> Self {
        self.record.logger_name = Some(logger_name);
        self
    }

    /// Sets the source location.
    // `Option` in the parameter is for the convenience of passing the result of
    // the macro `source_location_current` directly.
    #[must_use]
    pub(crate) fn source_location(mut self, srcloc: Option<SourceLocation>) -> Self {
        self.record.inner.to_mut().source_location = srcloc;
        self
    }

    /// Builds a [`Record`].
    pub(crate) fn build(self) -> Record<'a> {
        self.record
    }
}
