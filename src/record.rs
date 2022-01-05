//! Provides a log message structure.

use std::{
    borrow::{Borrow, Cow},
    time::SystemTime,
};

use crate::{Level, SourceLocation};

/// Represents a log message.
///
/// # Use
///
/// `Record` structures are passed as parameters to methods [`Logger::log`] and
/// [`Sink::log`]. Logger implementors forward these structures to its sinks,
/// then sink implementors manipulate these structures in order to process log
/// messages. `Record`s are automatically created by the [`log!`] macro and so
/// are not seen by log users.
///
/// [`Logger::log`]: crate::logger::Logger::log
/// [`Sink::log`]: crate::sink::Sink::log
/// [`log!`]: crate::log
#[derive(Clone, Debug)]
pub struct Record<'a> {
    logger_name: Option<&'a str>,
    level: Level,
    payload: Cow<'a, str>,
    source_location: Option<SourceLocation>,
    time: SystemTime,
}

impl<'a> Record<'a> {
    /// Constructs a `Record`.
    ///
    /// Typically users should only use it for testing [`Sink`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub fn new<S>(level: Level, payload: S) -> Record<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Record {
            logger_name: None,
            level,
            payload: payload.into(),
            source_location: None,
            time: SystemTime::now(),
        }
    }

    /// Constructs a [`RecordBuilder`].
    ///
    /// Typically users should only use it for testing [`Sink`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub fn builder<S>(level: Level, payload: S) -> RecordBuilder<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        RecordBuilder::new(level, payload)
    }

    /// The logger name.
    pub fn logger_name(&self) -> Option<&'a str> {
        self.logger_name
    }

    /// The verbosity level of the message.
    pub fn level(&self) -> Level {
        self.level
    }

    /// The payload of the message.
    pub fn payload(&self) -> &str {
        self.payload.borrow()
    }

    /// The source location of the message.
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    /// The time of the message.
    pub fn time(&self) -> &SystemTime {
        &self.time
    }
}

/// The builder of [`Record`].
///
/// Typically users should only use it for testing [`Sink`].
///
/// [`Sink`]: crate::sink::Sink
pub struct RecordBuilder<'a> {
    record: Record<'a>,
}

impl<'a> RecordBuilder<'a> {
    /// Constructs a `RecordBuilder`.
    ///
    /// The default value is the same as [`Record::new()`].
    ///
    /// Typically users should only use it for testing [`Sink`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub fn new<S>(level: Level, payload: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            record: Record::new(level, payload),
        }
    }

    /// Sets the logger name.
    pub fn logger_name(mut self, logger_name: &'a str) -> Self {
        self.record.logger_name = Some(logger_name);
        self
    }

    /// Sets the source location.
    // `Option` in the parameter is for the convenience of passing the result of
    // the macro `source_location_current` directly.
    pub fn source_location(mut self, srcloc: Option<SourceLocation>) -> Self {
        self.record.source_location = srcloc;
        self
    }

    /// Builds a [`Record`].
    pub fn build(self) -> Record<'a> {
        self.record
    }
}
