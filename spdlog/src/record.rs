use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
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
// FIXME: `Record` still owns some data and not just a reference, I'm not sure this is necessary and
// possible to correct.
#[derive(Clone, Debug)]
pub struct Record<'a> {
    logger_name: Option<Cow<'a, str>>,
    payload: Cow<'a, str>,
    inner: Cow<'a, RecordInner>,
}

#[derive(Clone, Debug)]
struct RecordInner {
    level: Level,
    source_location: Option<SourceLocation>,
    time: SystemTime,
    tid: u64,
}

impl<'a> Record<'a> {
    #[must_use]
    pub(crate) fn new(
        level: Level,
        payload: impl Into<Cow<'a, str>>,
        srcloc: Option<SourceLocation>,
        logger_name: Option<&'a str>,
    ) -> Record<'a> {
        Record {
            logger_name: logger_name.map(Cow::Borrowed),
            payload: payload.into(),
            inner: Cow::Owned(RecordInner {
                level,
                source_location: srcloc,
                time: SystemTime::now(),
                tid: get_current_tid(),
            }),
        }
    }

    /// Creates a [`RecordOwned`] that doesn't have lifetimes.
    #[must_use]
    pub fn to_owned(&self) -> RecordOwned {
        RecordOwned {
            logger_name: self.logger_name.clone().map(|n| n.into_owned()),
            payload: self.payload.to_string(),
            inner: self.inner.clone().into_owned(),
        }
    }

    /// Gets the logger name.
    #[must_use]
    pub fn logger_name(&self) -> Option<&str> {
        self.logger_name.as_ref().map(|n| n.as_ref())
    }

    /// Gets the level.
    #[must_use]
    pub fn level(&self) -> Level {
        self.inner.level
    }

    /// Gets the payload.
    #[must_use]
    pub fn payload(&self) -> &str {
        self.payload.borrow()
    }

    /// Gets the source location.
    #[must_use]
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.inner.source_location.as_ref()
    }

    /// Gets the time when the record was created.
    #[must_use]
    pub fn time(&self) -> SystemTime {
        self.inner.time
    }

    /// Gets the TID when the record was created.
    #[must_use]
    pub fn tid(&self) -> u64 {
        self.inner.tid
    }

    // When adding more getters, also add to `RecordOwned`

    #[must_use]
    pub(crate) fn replace_payload(&'a self, new: impl Into<Cow<'a, str>>) -> Self {
        Self {
            logger_name: self.logger_name.clone(),
            payload: new.into(),
            inner: Cow::Borrowed(&self.inner),
        }
    }

    #[cfg(feature = "log")]
    #[must_use]
    pub(crate) fn from_log_crate_record(
        logger: &'a crate::Logger,
        record: &log::Record,
        time: SystemTime,
    ) -> Self {
        let args = record.args();

        Self {
            // If the logger has a name configured, use that name. Otherwise, the name can also be
            // given by the target of the log record.
            logger_name: logger.name().map(Cow::Borrowed).or_else(|| {
                let log_target = record.target();
                if log_target.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(String::from(log_target)))
                }
            }),
            payload: match args.as_str() {
                Some(literal_str) => literal_str.into(),
                None => args.to_string().into(),
            },
            inner: Cow::Owned(RecordInner {
                level: record.level().into(),
                source_location: SourceLocation::from_log_crate_record(record),
                time,
                // For records from `log` crate, they never seem to come from different threads, so
                // getting the current TID here should be correct
                tid: get_current_tid(),
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
    /// References as [`Record`] cheaply.
    #[must_use]
    pub fn as_ref(&self) -> Record {
        Record {
            logger_name: self.logger_name.as_deref().map(Cow::Borrowed),
            payload: Cow::Borrowed(&self.payload),
            inner: Cow::Borrowed(&self.inner),
        }
    }

    /// Gets the logger name.
    #[must_use]
    pub fn logger_name(&self) -> Option<&str> {
        self.logger_name.as_deref()
    }

    /// Gets the level.
    #[must_use]
    pub fn level(&self) -> Level {
        self.inner.level
    }

    /// Gets the payload.
    #[must_use]
    pub fn payload(&self) -> &str {
        self.payload.borrow()
    }

    /// Gets the source location.
    #[must_use]
    pub fn source_location(&self) -> Option<&SourceLocation> {
        self.inner.source_location.as_ref()
    }

    /// Gets the time when the record was created.
    #[must_use]
    pub fn time(&self) -> SystemTime {
        self.inner.time
    }

    /// Gets the TID when the record was created.
    #[must_use]
    pub fn tid(&self) -> u64 {
        self.inner.tid
    }

    // When adding more getters, also add to `Record`
}

fn get_current_tid() -> u64 {
    #[cfg(target_os = "linux")]
    #[must_use]
    fn get_current_tid_inner() -> u64 {
        // https://github.com/SpriteOvO/spdlog-rs/issues/31
        //
        // We don't use `gettid` since earlier glibc versions (before v2.30) did not
        // provide a wrapper for this system call.
        let tid = unsafe { libc::syscall(libc::SYS_gettid) };
        tid as u64
    }

    #[cfg(target_os = "freebsd")]
    #[must_use]
    fn get_current_tid_inner() -> u64 {
        let tid = unsafe { libc::pthread_getthreadid_np() };
        tid as u64
    }

    #[cfg(target_os = "illumos")]
    #[must_use]
    fn get_current_tid_inner() -> u64 {
        let tid = unsafe { libc::thr_self() };
        tid as u64
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[must_use]
    fn get_current_tid_inner() -> u64 {
        let mut tid = 0;
        unsafe { libc::pthread_threadid_np(0, &mut tid) };
        tid
    }

    #[cfg(target_os = "windows")]
    #[must_use]
    fn get_current_tid_inner() -> u64 {
        let tid = unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() };
        tid as u64
    }

    thread_local! {
        static TID: RefCell<Option<u64>> = const { RefCell::new(None)} ;
    }

    TID.with(|tid| *tid.borrow_mut().get_or_insert_with(get_current_tid_inner))
}
