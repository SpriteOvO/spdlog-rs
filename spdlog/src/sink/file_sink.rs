//! Provides a file sink.

use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    path::PathBuf,
};

use crate::{
    sink::{helper, Sink},
    sync::*,
    utils, Error, Record, Result, StringBuf,
};

/// A sink with a file as the target.
///
/// # Examples
///
/// See [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct FileSink {
    common_impl: helper::CommonImpl,
    file: SpinMutex<BufWriter<File>>,
}

impl FileSink {
    /// Constructs a builder of `FileSink`.
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
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf)?;

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
///   # fn main() -> Result<(), spdlog::Error> {
///   let sink: FileSink = FileSink::builder()
///       .path("/path/to/log_file") // required
///       // .truncate(true) // optional, defaults to `false`
///       .build()?;
///   # Ok(()) }
///   ```
///
/// - If any required parameters are missing, a compile-time error will be
///   raised.
///
///   ```compile_fail,E0061
///   use spdlog::sink::FileSink;
///   
///   # fn main() -> Result<(), spdlog::Error> {
///   let sink: FileSink = FileSink::builder()
///       // .path("/path/to/log_file") // required
///       .truncate(true) // optional, defaults to `false`
///       .build()?;
///   # Ok(()) }
///   ```
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

    /// If it is true, the existing contents of the filewill be discarded.
    ///
    /// This parameter is **optional**, and defaults to `false`.
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
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
            file: SpinMutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}
