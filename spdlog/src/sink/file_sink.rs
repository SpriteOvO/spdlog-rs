//! Provides a file sink.

use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Write as _},
    path::{Path, PathBuf},
};

use crate::{
    formatter::{Formatter, FormatterContext},
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    utils, Error, ErrorHandler, LevelFilter, Record, Result, StringBuf,
};

/// A sink with a file as the target.
///
/// It writes logs to a single file. If you want to automatically rotate into
/// multiple files, see  [`RotatingFileSink`].
///
/// The file and directories will be created recursively if they do not exist.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [`RotatingFileSink`]: crate::sink::RotatingFileSink
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct FileSink {
    prop: SinkProp,
    file: Mutex<BufWriter<File>>,
}

impl FileSink {
    /// Gets a builder of `FileSink` with default parameters:
    ///
    /// | Parameter       | Default Value               |
    /// |-----------------|-----------------------------|
    /// | [level_filter]  | [`LevelFilter::All`]        |
    /// | [formatter]     | [`FullFormatter`]           |
    /// | [error_handler] | [`ErrorHandler::default()`] |
    /// |                 |                             |
    /// | [path]          | *must be specified*         |
    /// | [truncate]      | `false`                     |
    /// | [capacity]      | consistent with `std`       |
    ///
    /// [level_filter]: FileSinkBuilder::level_filter
    /// [formatter]: FileSinkBuilder::formatter
    /// [`FullFormatter`]: crate::formatter::FullFormatter
    /// [error_handler]: FileSinkBuilder::error_handler
    /// [path]: FileSinkBuilder::path
    /// [truncate]: FileSinkBuilder::truncate
    /// [capacity]: FileSinkBuilder::capacity
    #[must_use]
    pub fn builder() -> FileSinkBuilder<()> {
        FileSinkBuilder {
            prop: SinkProp::default(),
            path: (),
            truncate: false,
            capacity: None,
        }
    }

    /// Constructs a `FileSink`.
    ///
    /// If the parameter `truncate` is `true`, the existing contents of the file
    /// will be discarded.
    ///
    /// # Error
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `FileSink::builder()` instead"
    )]
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

impl GetSinkProp for FileSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for FileSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        self.file
            .lock_expect()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock_expect().flush().map_err(Error::FlushBuffer)
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Err(err) = self.file.lock_expect().flush() {
            self.prop
                .call_error_handler_internal("FileSink", Error::FlushBuffer(err))
        }
    }
}

// --------------------------------------------------

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct FileSinkBuilder<ArgPath> {
    prop: SinkProp,
    path: ArgPath,
    truncate: bool,
    capacity: Option<usize>,
}

impl<ArgPath> FileSinkBuilder<ArgPath> {
    /// The path of the log file.
    ///
    /// This parameter is **required**.
    #[must_use]
    pub fn path<P>(self, path: P) -> FileSinkBuilder<PathBuf>
    where
        P: Into<PathBuf>,
    {
        FileSinkBuilder {
            prop: self.prop,
            path: path.into(),
            truncate: self.truncate,
            capacity: self.capacity,
        }
    }

    /// Truncates the contents when opening an existing file.
    ///
    /// If it is `true`, the existing contents of the file will be discarded.
    ///
    /// This parameter is **optional**, and defaults to `false`.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    /// Specifies the internal buffer capacity.
    ///
    /// This parameter is **optional**, and defaults to the value consistent
    /// with `std`.
    #[must_use]
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

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
    /// This parameter is **optional**, and defaults to [`FullFormatter`].
    ///
    /// [`FullFormatter`]: crate::formatter::FullFormatter
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
}

impl FileSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `path`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}

    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `path`\n\n\
    ")]
    pub fn build_arc(self, _: Infallible) {}
}

impl FileSinkBuilder<PathBuf> {
    /// Builds a [`FileSink`].
    ///
    /// # Error
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    pub fn build(self) -> Result<FileSink> {
        let file = utils::open_file_bufw(self.path, self.truncate, self.capacity)?;

        let sink = FileSink {
            prop: self.prop,
            file: Mutex::new(file),
        };

        Ok(sink)
    }

    /// Builds a `Arc<FileSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<FileSink>> {
        self.build().map(Arc::new)
    }
}
