//! Provides a file sink.

use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Write},
    mem,
    path::Path,
    path::PathBuf,
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
    /// Constructs a builder of `FileSink`.
    pub fn builder() -> FileSinkBuilder<()> {
        FileSinkBuilder {
            path: (),
            truncate: false,
        }
    }

    /// Constructs a `FileSink`.
    ///
    /// If the parameter `truncate` is `true`, the existing contents of the file
    /// will be discarded.
    ///
    /// # Errors
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    #[deprecated(note = "it may be removed in the future, use `FileSink::builder()` instead")]
    pub fn new<P>(path: P, truncate: bool) -> Result<FileSink>
    where
        P: AsRef<Path>, /* Keep the `AsRef<Path>` instead of `Into<PathBuf>` for backward
                         * compatible */
    {
        Self::builder()
            .path(path.as_ref())
            .truncate(truncate)
            .build()
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

// --------------------------------------------------

/// The builder of [`FileSink`].
#[doc = include_str!("../include/doc/generic-builder-note.md")]
///
/// # Examples
///
/// - Building a [`FileSink`].
///
///   ```no_run
///   use spdlog::sink::FileSink;
///  
///   let sink: spdlog::Result<FileSink> = FileSink::builder()
///       .path("/path/to/log_file") // required
///       // .truncate(true) // optional, defaults to `false`
///       .build();
///   ```
///
/// - If any required parameters are missing, a compile-time error will be
///   raised.
///
///   ```compile_fail
///   use spdlog::sink::FileSink;
///   
///   let sink: spdlog::Result<FileSink> = FileSink::builder()
///       // .path("/path/to/log_file") // required
///       .truncate(true) // optional, defaults to `false`
///       .build();
///   ```
pub struct FileSinkBuilder<ArgPath> {
    path: ArgPath,
    truncate: bool,
}

impl<ArgPath> FileSinkBuilder<ArgPath> {
    /// The path of the log file.
    ///
    /// This parameter is required.
    pub fn path<P>(self, path: P) -> FileSinkBuilder<PathBuf>
    where
        P: Into<PathBuf>,
    {
        FileSinkBuilder {
            path: path.into(),
            truncate: self.truncate,
        }
    }

    /// If it is true, the existing contents of the filewill be discarded.
    ///
    /// This parameter is optional, and defaults to `false`.
    pub fn truncate(self, truncate: bool) -> Self {
        FileSinkBuilder { truncate, ..self }
    }
}

impl FileSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `path`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl FileSinkBuilder<PathBuf> {
    /// Builds a [`FileSink`].
    ///
    /// # Errors
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    pub fn build(self) -> Result<FileSink> {
        let file = utils::open_file(self.path, self.truncate)?;

        let sink = FileSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            file: SpinMutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}
