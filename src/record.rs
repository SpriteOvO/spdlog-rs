//! Provides a log message structure.

use chrono::prelude::*;

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
    level: Level,
    payload: &'a str,
    source_location: Option<SourceLocation>,
    time: DateTime<Utc>,
}

impl<'a> Record<'a> {
    /// Constructs a `Record`.
    ///
    /// Typically users should only use it for testing [`Logger`] or [`Sink`].
    ///
    /// [`Logger`]: crate::logger::Logger
    /// [`Sink`]: crate::sink::Sink
    pub fn new(level: Level, payload: &'a str) -> Record {
        Record {
            level,
            payload,
            source_location: None,
            time: Utc::now(),
        }
    }

    /// Constructs a `Record` with a [`SourceLocation`].
    ///
    /// Typically users should only use it for testing [`Logger`] or [`Sink`].
    ///
    /// [`Logger`]: crate::logger::Logger
    /// [`Sink`]: crate::sink::Sink
    pub fn with_source_location(
        level: Level,
        payload: &'a str,
        source_location: SourceLocation,
    ) -> Record {
        Record {
            level,
            payload,
            source_location: Some(source_location),
            time: Utc::now(),
        }
    }

    /// The verbosity level of the message.
    pub fn level(&self) -> Level {
        self.level
    }

    /// The payload of the message.
    pub fn payload(&self) -> &str {
        self.payload
    }

    /// The source location of the message.
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    /// The time of the message.
    pub fn time(&self) -> &DateTime<Utc> {
        &self.time
    }
}
