use std::{convert::Infallible, io::Write, marker::PhantomData};

use crate::{
    sink::{helper, Sink},
    sync::*,
    Error, Record, Result, StringBuf,
};

/// A sink that writes log messages into an arbitrary `impl Write` object.
///
/// # Performance Notice
///
/// Since `WriteSink` can write into any `impl Write` objects, the assumptions
/// made on the underlying `impl Write` object is very weak and this does impact
/// performance. You should use other sinks or implement your own sinks whenever
/// possible. `WriteSink` is your last resort if no other sinks meet your
/// requirement.
///
/// If you want to log into a file, use [`FileSink`] or [`RotatingFileSink`]
/// instead.
///
/// If you want to log into the standard streams, use [`StdStreamSink`] instead.
///
/// [`FileSink`]: crate::sink::FileSink
/// [`RotatingFileSink`]: crate::sink::RotatingFileSink
/// [`StdStreamSink`]: crate::sink::StdStreamSink
pub struct WriteSink<W>
where
    W: Write + Send,
{
    common_impl: helper::CommonImpl,
    target: Mutex<W>,
}

impl<W> WriteSink<W>
where
    W: Write + Send,
{
    /// Constructs a builder of `WriteSink`.
    pub fn builder() -> WriteSinkBuilder<W, ()> {
        WriteSinkBuilder {
            common_builder_impl: helper::CommonBuilderImpl::new(),
            target: None,
            _phantom: PhantomData,
        }
    }

    /// Invoke a callback function with the underlying `impl Write` object.
    ///
    /// This function returns whatever the given callback function returns.
    ///
    /// Note that this sink cannot write into the underlying `impl Write` object
    /// while the given callback function is running. If the underlying
    /// `impl Write` object supports a relatively cheap `clone` operation,
    /// consider using the [`clone_target`] method.
    ///
    /// [`clone_target`]: Self::clone_target
    pub fn with_target<F, R>(&self, callback: F) -> R
    where
        F: FnOnce(&mut W) -> R,
    {
        callback(&mut *self.lock_target())
    }

    fn lock_target(&self) -> MutexGuard<W> {
        self.target.lock_expect()
    }
}

impl<W> WriteSink<W>
where
    W: Write + Send + Clone,
{
    /// Clone the underlying `impl Write` object.
    pub fn clone_target(&self) -> W {
        self.lock_target().clone()
    }
}

impl<W> Sink for WriteSink<W>
where
    W: Write + Send,
{
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();
        self.common_impl
            .formatter
            .read()
            .format(record, &mut string_buf)?;

        self.lock_target()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.lock_target().flush().map_err(Error::FlushBuffer)
    }

    helper::common_impl!(@Sink: common_impl);
}

impl<W> Drop for WriteSink<W>
where
    W: Write + Send,
{
    fn drop(&mut self) {
        let flush_result = self.lock_target().flush().map_err(Error::FlushBuffer);
        if let Err(err) = flush_result {
            self.common_impl.non_returnable_error("WriteSink", err)
        }
    }
}

/// The builder of [`WriteSink`].
#[doc = include_str!("../include/doc/generic-builder-note.md")]
///
/// # Examples
///
/// - Building a [`WriteSink`].
///
///   ```
///   use spdlog::{prelude::*, sink::WriteSink};
///  
///   # fn main() -> Result<(), spdlog::Error> {
///   # let target = Vec::new();
///   let sink: WriteSink<_> = WriteSink::builder()
///       .target(target) // required
///       // .level_filter(LevelFilter::MoreSevere(Level::Info)) // optional
///       .build()?;
///   # Ok(()) }
///   ```
///
/// - If any required parameters are missing, a compile-time error will be
///   raised.
///
///   ```compile_fail,E0061
///   use spdlog::{prelude::*, sink::WriteSink};
///  
///   # fn main() -> Result<(), spdlog::Error> {
///   let sink: WriteSink<_> = WriteSink::builder()
///       // .target(target) // required
///       .level_filter(LevelFilter::MoreSevere(Level::Info)) // optional
///       .build()?;
///   # Ok(()) }
///   ```
pub struct WriteSinkBuilder<W, ArgW> {
    common_builder_impl: helper::CommonBuilderImpl,
    target: Option<W>,
    _phantom: PhantomData<ArgW>,
}

impl<W, ArgW> WriteSinkBuilder<W, ArgW>
where
    W: Write + Send,
{
    /// Specifies the target that implemented [`Write`] trait, log messages will
    /// be written into the target.
    ///
    /// This parameter is required.
    pub fn target(self, target: W) -> WriteSinkBuilder<W, PhantomData<W>> {
        WriteSinkBuilder {
            common_builder_impl: self.common_builder_impl,
            target: Some(target),
            _phantom: PhantomData,
        }
    }

    helper::common_impl!(@SinkBuilder: common_builder_impl);
}

impl<W> WriteSinkBuilder<W, ()>
where
    W: Write + Send,
{
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `target`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl<W> WriteSinkBuilder<W, PhantomData<W>>
where
    W: Write + Send,
{
    /// Builds a [`WriteSink`].
    pub fn build(self) -> Result<WriteSink<W>> {
        let sink = WriteSink {
            common_impl: helper::CommonImpl::from_builder(self.common_builder_impl),
            target: Mutex::new(self.target.unwrap()),
        };
        Ok(sink)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::*};

    #[test]
    fn validation() {
        let sink = Arc::new(WriteSink::builder().target(Vec::new()).build().unwrap());
        sink.set_formatter(Box::new(NoModFormatter::new()));
        let logger = test_logger_builder()
            .sink(sink.clone())
            .level_filter(LevelFilter::All)
            .build();

        info!(logger: logger, "hello WriteSink");

        let data = sink.clone_target();
        assert_eq!(data.as_slice(), b"hello WriteSink");
    }
}
