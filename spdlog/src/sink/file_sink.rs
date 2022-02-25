//! Provides a file sink.

use std::{
    fs::File,
    io::{BufWriter, Write},
    mem,
    path::Path,
};

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    sync::*,
    utils, Error, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a file as the target.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct FileSink {
    level_filter: Atomic<LevelFilter>,
    formatter: SpinRwLock<Box<dyn Formatter>>,
    file: SpinMutex<BufWriter<File>>,
}

impl FileSink {
    /// Constructs a `FileSink`.
    ///
    /// If the parameter `truncate` is `true`, the existing contents of the file
    /// will be discarded.
    ///
    /// # Errors
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    pub fn new<P>(path: P, truncate: bool) -> Result<FileSink>
    where
        P: AsRef<Path>,
    {
        let file = utils::open_file(path, truncate)?;

        let sink = FileSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            file: SpinMutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}

impl Sink for FileSink {
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.formatter.read().format(record, &mut string_buf)?;

        self.file
            .lock()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock().flush().map_err(Error::FlushBuffer)
    }

    fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    fn swap_formatter(&self, mut formatter: Box<dyn Formatter>) -> Box<dyn Formatter> {
        mem::swap(&mut *self.formatter.write(), &mut formatter);
        formatter
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Err(err) = self.file.lock().flush() {
            // Sinks do not have an error handler, because it would increase complexity and
            // the error is not common. So currently users cannot handle this error by
            // themselves.
            crate::default_error_handler("FileSink", Error::FlushBuffer(err));
        }
    }
}
