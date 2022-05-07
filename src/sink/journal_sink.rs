use std::{io, os::raw::c_int};

use libsystemd_sys::{const_iovec, journal as ffi};

use crate::{
    formatter::JournalFormatter,
    sink::{helper, Sink},
    Error, Level, Record, Result, StdResult, StringBuf,
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
    let iovecs: Vec<_> = args.map(|a| unsafe { const_iovec::from_str(a) }).collect();
    let result = unsafe { ffi::sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(result))
    }
}

/// A sink with systemd journal as the target.
///
/// # Log Level Mapping
///
/// | spdlog-rs  |  journal  |
/// |------------|-----------|
/// | `Critical` | `crit`    |
/// | `Error`    | `err`     |
/// | `Warn`     | `warning` |
/// | `Info`     | `info`    |
/// | `Debug`    | `debug`   |
/// | `Trace`    | `debug`   |
pub struct JournalSink {
    common_impl: helper::CommonImpl,
}

impl JournalSink {
    const SYSLOG_LEVELS: SyslogLevels = SyslogLevels::new();

    /// Constructs a `JournalSink`.
    pub fn new() -> Self {
        Self {
            common_impl: helper::CommonImpl::with_formatter(Box::new(JournalFormatter::new())),
        }
    }
}

impl Sink for JournalSink {
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf)?;

        let kvs = [
            format!("MESSAGE={}", string_buf),
            format!(
                "PRIORITY={}",
                JournalSink::SYSLOG_LEVELS.level(record.level()) as u32
            ),
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

    helper::common_impl!(@Sink: common_impl);
}

impl Default for JournalSink {
    fn default() -> Self {
        Self::new()
    }
}
