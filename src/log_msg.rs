//! Provides the log message structure.

use std::borrow::Cow;

use chrono::prelude::*;
pub use log::{Metadata, Record};

use crate::Level;

/// Represents a log message.
pub struct LogMsg<'a> {
    level: Level,
    target: &'a str,
    time: DateTime<Utc>,
    payload: Cow<'a, str>,
}

impl<'a> LogMsg<'a> {
    /// Constructs a [`LogMsg`] from a [`Record`].
    pub fn new(record: &'a Record) -> LogMsg<'a> {
        LogMsg {
            level: record.level(),
            target: record.target(),
            time: Utc::now(),
            payload: record.args().to_string().into(),
        }
    }

    /// Getter of the level
    pub fn level(&self) -> Level {
        self.level
    }

    /// Getter of the target
    pub fn target(&self) -> &'a str {
        self.target
    }

    /// Getter of the time
    pub fn time(&self) -> &DateTime<Utc> {
        &self.time
    }

    /// Getter of the payload
    pub fn payload(&self) -> &Cow<'a, str> {
        &self.payload
    }
}
