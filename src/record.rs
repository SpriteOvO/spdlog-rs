//! Provides a log message structure.

use std::borrow::{Borrow, Cow};

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
    payload: Cow<'a, str>,
    source_location: Option<SourceLocation>,
    time: DateTime<Utc>,
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
            level,
            payload: payload.into(),
            source_location: None,
            time: Utc::now(),
        }
    }

    /// Constructs a `Record` with a [`SourceLocation`].
    ///
    /// Typically users should only use it for testing [`Sink`].
    ///
    /// [`Sink`]: crate::sink::Sink
    pub fn with_source_location<S>(
        level: Level,
        payload: S,
        source_location: Option<SourceLocation>,
    ) -> Record<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Record {
            level,
            payload: payload.into(),
            source_location,
            time: Utc::now(),
        }
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
    pub fn time(&self) -> &DateTime<Utc> {
        &self.time
    }
}
