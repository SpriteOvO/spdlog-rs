//! Provides a file sink.

use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    formatter::FormatterContext,
    sink::{helper, Sink},
    sync::*,
    utils, Error, Record, Result, StringBuf,
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
    common_impl: helper::CommonImpl,
    file: SpinMutex<BufWriter<File>>,
}

impl FileSink {
    /// Gets a builder of `FileSink` with default parameters:
    ///
    /// | Parameter       | Default Value           |
    /// |-----------------|-------------------------|
    /// | [level_filter]  | `All`                   |
    /// | [formatter]     | `FullFormatter`         |
    /// | [error_handler] | [default error handler] |
    /// |                 |                         |
    /// | [path]          | *must be specified*     |
    /// | [truncate]      | `false`                 |
    ///
    /// [level_filter]: FileSinkBuilder::level_filter
    /// [formatter]: FileSinkBuilder::formatter
    /// [error_handler]: FileSinkBuilder::error_handler
    /// [default error handler]: error/index.html#default-error-handler
    /// [path]: FileSinkBuilder::path
    /// [truncate]: FileSinkBuilder::truncate
    #[must_use]
    pub fn builder() -> FileSinkBuilder<()> {
        FileSinkBuilder {
            path: (),
            truncate: false,
            common_builder_impl: helper::CommonBuilderImpl::new(),
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

impl Sink for FileSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf, &mut ctx)?;

        self.file
            .lock()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.file.lock().flush().map_err(Error::FlushBuffer)
    }

    helper::common_impl!(@Sink: common_impl);
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Err(err) = self.file.lock().flush() {
            self.common_impl
                .non_returnable_error("FileSink", Error::FlushBuffer(err))
        }
    }
}

// --------------------------------------------------

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct FileSinkBuilder<ArgPath> {
    common_builder_impl: helper::CommonBuilderImpl,
    path: ArgPath,
    truncate: bool,
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
            common_builder_impl: self.common_builder_impl,
            path: path.into(),
            truncate: self.truncate,
        }
    }

    /// Truncates the contents when opening an existing file.
    ///
    /// If it is `true`, the existing contents of the file will be discarded.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    helper::common_impl!(@SinkBuilder: common_builder_impl);
}

impl FileSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `path`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl FileSinkBuilder<PathBuf> {
    /// Builds a [`FileSink`].
    ///
    /// # Error
    ///
    /// If an error occurs opening the file, [`Error::CreateDirectory`] or
    /// [`Error::OpenFile`] will be returned.
    pub fn build(self) -> Result<FileSink> {
        let file = utils::open_file(self.path, self.truncate)?;

        let sink = FileSink {
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
            file: SpinMutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}
