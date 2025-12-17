use std::{io, os::raw::c_int};

use crate::{
    formatter::{Formatter, FormatterContext, FullFormatter},
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    Error, ErrorHandler, Level, LevelFilter, Record, Result, StdResult, StringBuf,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum SyslogLevel {
    _Emerg = 0,
    _Alert = 1,
    Crit = 2,
    Err = 3,
    Warning = 4,
    _Notice = 5,
    Info = 6,
    Debug = 7,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct SyslogLevels([SyslogLevel; Level::count()]);

impl SyslogLevels {
    #[must_use]
    const fn new() -> Self {
        Self([
            SyslogLevel::Crit,    // Critical
            SyslogLevel::Err,     // Error
            SyslogLevel::Warning, // Warn
            SyslogLevel::Info,    // Info
            SyslogLevel::Debug,   // Debug
            SyslogLevel::Debug,   // Trace
        ])
    }

    #[must_use]
    fn level(&self, level: Level) -> SyslogLevel {
        self.0[level as usize]
    }
}

impl Default for SyslogLevels {
    fn default() -> Self {
        Self::new()
    }
}

fn journal_send(args: impl Iterator<Item = impl AsRef<str>>) -> StdResult<(), io::Error> {
    #[cfg(not(doc))] // https://github.com/rust-lang/rust/issues/97976
    use libsystemd_sys::{const_iovec, journal as ffi};

    let iovecs: Vec<_> = args.map(|a| unsafe { const_iovec::from_str(a) }).collect();
    let result = unsafe { ffi::sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(result))
    }
}

/// A sink with systemd-journal as the target.
///
/// # Log Level Mapping
///
/// | spdlog-rs  | journald  |
/// |------------|-----------|
/// | `Critical` | `crit`    |
/// | `Error`    | `err`     |
/// | `Warn`     | `warning` |
/// | `Info`     | `info`    |
/// | `Debug`    | `debug`   |
/// | `Trace`    | `debug`   |
///
/// # Note
///
/// It requires an additional system dependency `libsystemd`.
///
/// ## Install on Ubuntu / Debian
///
/// ```bash
/// apt install libsystemd-dev
/// ```
///
/// ## Install on ArchLinux
///
/// ```bash
/// pacman -S systemd
/// ```
pub struct JournaldSink {
    prop: SinkProp,
}

impl JournaldSink {
    const SYSLOG_LEVELS: SyslogLevels = SyslogLevels::new();

    /// Gets a builder of `JournaldSink` with default parameters:
    ///
    /// | Parameter       | Default Value                                |
    /// |-----------------|----------------------------------------------|
    /// | [level_filter]  | [`LevelFilter::All`]                         |
    /// | [formatter]     | [`FullFormatter`] `(!time !source_location)` |
    /// | [error_handler] | [`ErrorHandler::default()`]                  |
    ///
    /// [level_filter]: JournaldSinkBuilder::level_filter
    /// [formatter]: JournaldSinkBuilder::formatter
    /// [error_handler]: JournaldSinkBuilder::error_handler
    #[must_use]
    pub fn builder() -> JournaldSinkBuilder {
        let prop = SinkProp::default();
        prop.set_formatter(
            FullFormatter::builder()
                .time(false)
                .source_location(false)
                .build(),
        );

        JournaldSinkBuilder { prop }
    }
}

impl GetSinkProp for JournaldSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for JournaldSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        let kvs = [
            format!("MESSAGE={string_buf}"),
            format!(
                "PRIORITY={}",
                JournaldSink::SYSLOG_LEVELS.level(record.level()) as u32
            ),
            format!("TID={}", record.tid()),
        ];

        let srcloc_kvs = match record.source_location() {
            Some(srcloc) => [
                Some(format!("CODE_FILE={}", srcloc.file_name())),
                Some(format!("CODE_LINE={}", srcloc.line())),
            ],
            None => [None, None],
        };

        journal_send(kvs.iter().chain(srcloc_kvs.iter().flatten())).map_err(Error::WriteRecord)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

#[allow(missing_docs)]
pub struct JournaldSinkBuilder {
    prop: SinkProp,
}

impl JournaldSinkBuilder {
    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**, and defaults to [`LevelFilter::All`].
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    /// Specifies a formatter.
    ///
    /// This parameter is **optional**, and defaults to [`FullFormatter`]
    /// `(!time !source_location)`.
    #[must_use]
    pub fn formatter<F>(self, formatter: F) -> Self
    where
        F: Formatter + 'static,
    {
        self.prop.set_formatter(formatter);
        self
    }

    /// Specifies an error handler.
    ///
    /// This parameter is **optional**, and defaults to
    /// [`ErrorHandler::default()`].
    #[must_use]
    pub fn error_handler<F: Into<ErrorHandler>>(self, handler: F) -> Self {
        self.prop.set_error_handler(handler);
        self
    }

    //

    /// Builds a [`JournaldSink`].
    pub fn build(self) -> Result<JournaldSink> {
        let sink = JournaldSink { prop: self.prop };
        Ok(sink)
    }

    /// Builds a `Arc<JournaldSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<JournaldSink>> {
        self.build().map(Arc::new)
    }
}
