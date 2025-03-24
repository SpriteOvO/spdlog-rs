use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    time::SystemTime,
};

use crate::{kv, Level, SourceLocation};

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
    logger_name: Option<&'a str>,
    payload: Cow<'a, str>,
    kvs: Cow<'a, [kv::Pair<'a>]>,
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
        kvs: &'a [(kv::Key<'a>, kv::Value<'a>)],
    ) -> Record<'a> {
        Record {
            logger_name,
            payload: payload.into(),
            kvs: Cow::Borrowed(kvs),
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
            logger_name: self.logger_name.map(|n| n.to_owned()),
            payload: self.payload.to_string(),
            kvs: self
                .kvs
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
            inner: self.inner.clone().into_owned(),
        }
    }

    /// Gets the logger name.
    #[must_use]
    pub fn logger_name(&self) -> Option<&str> {
        self.logger_name
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

    /// Gets the key-values.
    #[must_use]
    pub fn key_values(&self) -> kv::KeyValues {
        kv::KeyValues::with_borrowed(&self.kvs)
    }

    // When adding more getters, also add to `RecordOwned`

    #[must_use]
    pub(crate) fn replace_payload(&'a self, new: impl Into<Cow<'a, str>>) -> Self {
        Self {
            logger_name: self.logger_name,
            payload: new.into(),
            kvs: self.kvs.clone(),
            inner: Cow::Borrowed(&self.inner),
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
    kvs: Vec<(kv::KeyOwned, kv::ValueOwned)>,
    inner: RecordInner,
}

impl RecordOwned {
    /// References as [`Record`] cheaply.
    #[must_use]
    pub fn as_ref(&self) -> Record {
        Record {
            logger_name: self.logger_name.as_deref(),
            payload: Cow::Borrowed(&self.payload),
            kvs: Cow::Owned(
                self.kvs
                    .iter()
                    .map(|(k, v)| (k.as_ref(), v.by_ref()))
                    .collect::<Vec<_>>(),
            ),
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

    /// Gets the key-values.
    #[must_use]
    pub fn key_values(&self) -> kv::KeyValues {
        kv::KeyValues::with_owned(&self.kvs)
    }

    // When adding more getters, also add to `Record`
}

#[cfg(feature = "log")]
#[derive(Clone, Debug)]
pub(crate) struct LogCrateRecord<'a> {
    logger_name: Option<&'a str>,
    payload: Cow<'a, str>,
    kvs: Vec<(log::kv::Key<'a>, kv::ValueOwned)>,
    inner: Cow<'a, RecordInner>,
}

#[cfg(feature = "log")]
impl<'a> LogCrateRecord<'a> {
    #[must_use]
    pub(crate) fn new(
        logger: &'a crate::Logger,
        record: &'a log::Record,
        time: SystemTime,
    ) -> Self {
        let args = record.args();

        Self {
            // If the logger has a name configured, use that name. Otherwise, the name can also be
            // given by the target of the log record.
            logger_name: logger.name().or_else(|| Some(record.target())),
            kvs: {
                let kvs = record.key_values();
                let mut cvt = kv::LogCrateConverter::new(kvs.count());
                assert!(kvs.visit(&mut cvt).is_ok());
                cvt.finalize()
            },
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

    #[must_use]
    pub(crate) fn as_record(&self) -> Record {
        Record {
            logger_name: self.logger_name,
            payload: self.payload.clone(),
            kvs: self
                .kvs
                .iter()
                .map(|(k, v)| (kv::Key::from_str(k.as_str()), v.by_ref()))
                .collect(),
            inner: self.inner.clone(),
        }
    }
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
