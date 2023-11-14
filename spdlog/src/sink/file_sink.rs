//! Provides a file sink.

use std::{
    convert::Infallible,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{
    config::{ComponentMetadata, Configurable},
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
            inner: FileSinkParamsInner {
                path: (),
                truncate: false,
                common_builder_impl: helper::CommonBuilderImpl::new(),
            },
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

#[derive(Default, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
struct FileSinkParamsInner<ArgPath> {
    #[serde(flatten)]
    common_builder_impl: helper::CommonBuilderImpl,
    path: ArgPath,
    #[serde(default)]
    truncate: bool,
}

#[derive(Default, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[doc(hidden)]
pub struct FileSinkParams(FileSinkParamsInner<PathBuf>);

impl Configurable for FileSink {
    type Params = FileSinkParams;

    fn metadata() -> ComponentMetadata<'static> {
        ComponentMetadata { name: "FileSink" }
    }

    fn build(params: Self::Params) -> Result<Self> {
        let mut builder = FileSink::builder()
            .level_filter(params.0.common_builder_impl.level_filter)
            // .error_handler(params.0.common_builder_impl.error_handler)
            .path(params.0.path)
            .truncate(params.0.truncate);
        if let Some(formatter) = params.0.common_builder_impl.formatter {
            builder = builder.formatter(formatter);
        }
        builder.build()
    }
}

// --------------------------------------------------

/// The builder of [`FileSink`].
#[doc = include_str!("../include/doc/generic-builder-note.md")]
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
    inner: FileSinkParamsInner<ArgPath>,
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
            inner: FileSinkParamsInner {
                common_builder_impl: self.inner.common_builder_impl,
                path: path.into(),
                truncate: self.inner.truncate,
            },
        }
    }

    /// If it is true, the existing contents of the filewill be discarded.
    ///
    /// This parameter is **optional**, and defaults to `false`.
    #[must_use]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.inner.truncate = truncate;
        self
    }

    helper::common_impl!(@SinkBuilder: inner.common_builder_impl);
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
        let file = utils::open_file(self.inner.path, self.inner.truncate)?;

        let sink = FileSink {
            common_impl: helper::CommonImpl::from_builder(self.inner.common_builder_impl),
            file: SpinMutex::new(BufWriter::new(file)),
        };

        Ok(sink)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        formatter::{FullFormatter, PatternFormatter, RuntimePattern},
        LevelFilter,
    };

    #[test]
    fn deser_params() {
        assert!(
            toml::from_str::<FileSinkParams>(r#"path = "/path/to/log_file""#,).unwrap()
                == FileSinkParams(FileSinkParamsInner {
                    common_builder_impl: helper::CommonBuilderImpl {
                        level_filter: LevelFilter::All,
                        formatter: None,
                        error_handler: None
                    },
                    path: PathBuf::from_str("/path/to/log_file").unwrap(),
                    truncate: false
                })
        );
        assert!(
            toml::from_str::<FileSinkParams>(
                r#"
                    path = "/path/to/app.log"
                    truncate = true
                    formatter = { name = "FullFormatter" }
                "#,
            )
            .unwrap()
                == FileSinkParams(FileSinkParamsInner {
                    common_builder_impl: helper::CommonBuilderImpl {
                        level_filter: LevelFilter::All,
                        formatter: Some(Box::new(FullFormatter::new())),
                        error_handler: None
                    },
                    path: PathBuf::from_str("/path/to/app.log").unwrap(),
                    truncate: true
                })
        );
        assert!(
            toml::from_str::<FileSinkParams>(
                r#"
                    path = "/path/to/app.log"
                    truncate = true
                    formatter = { name = "PatternFormatter", template = "[{level}] >w< {payload}{eol}" }
                "#,
            )
            .unwrap()
                == FileSinkParams(FileSinkParamsInner {
                    common_builder_impl: helper::CommonBuilderImpl {
                        level_filter: LevelFilter::All,
                        formatter: Some(Box::new(PatternFormatter::new(
                            RuntimePattern::new("[{level}] >w< {payload}{eol}").unwrap()
                        ))),
                        error_handler: None
                    },
                    path: PathBuf::from_str("/path/to/app.log").unwrap(),
                    truncate: true
                })
        );
    }
}
