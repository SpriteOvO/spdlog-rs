use std::{convert::Infallible, io::Write, marker::PhantomData};

use crate::{
    formatter::{Formatter, FormatterContext},
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    Error, ErrorHandler, LevelFilter, Record, Result, StringBuf,
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
    prop: SinkProp,
    target: Mutex<W>,
}

impl<W> WriteSink<W>
where
    W: Write + Send,
{
    /// Gets a builder of `WriteSink` with default parameters:
    ///
    /// | Parameter         | Default Value               |
    /// |-------------------|-----------------------------|
    /// | [level_filter]    | `All`                       |
    /// | [formatter]       | `FullFormatter`             |
    /// | [error_handler]   | [`ErrorHandler::default()`] |
    /// |                   |                             |
    /// | [target]          | *must be specified*         |
    ///
    /// [level_filter]: WriteSinkBuilder::level_filter
    /// [formatter]: WriteSinkBuilder::formatter
    /// [error_handler]: WriteSinkBuilder::error_handler
    /// [`ErrorHandler::default()`]: crate::error::ErrorHandler::default()
    /// [target]: WriteSinkBuilder::target
    #[must_use]
    pub fn builder() -> WriteSinkBuilder<W, ()> {
        WriteSinkBuilder {
            prop: SinkProp::default(),
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
    #[must_use]
    pub fn with_target<F, R>(&self, callback: F) -> R
    where
        F: FnOnce(&mut W) -> R,
    {
        callback(&mut *self.lock_target())
    }

    fn lock_target(&self) -> MutexGuard<'_, W> {
        self.target.lock_expect()
    }
}

impl<W> WriteSink<W>
where
    W: Write + Send + Clone,
{
    /// Clone the underlying `impl Write` object.
    #[must_use]
    pub fn clone_target(&self) -> W {
        self.lock_target().clone()
    }
}

impl<W> GetSinkProp for WriteSink<W>
where
    W: Write + Send,
{
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl<W> Sink for WriteSink<W>
where
    W: Write + Send,
{
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        self.lock_target()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.lock_target().flush().map_err(Error::FlushBuffer)
    }
}

impl<W> Drop for WriteSink<W>
where
    W: Write + Send,
{
    fn drop(&mut self) {
        let flush_result = self.lock_target().flush().map_err(Error::FlushBuffer);
        if let Err(err) = flush_result {
            self.prop.call_error_handler_internal("WriteSink", err)
        }
    }
}

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct WriteSinkBuilder<W, ArgW> {
    prop: SinkProp,
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
    /// This parameter is **required**.
    #[must_use]
    pub fn target(self, target: W) -> WriteSinkBuilder<W, PhantomData<W>> {
        WriteSinkBuilder {
            prop: self.prop,
            target: Some(target),
            _phantom: PhantomData,
        }
    }

    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    /// Specifies a formatter.
    ///
    /// This parameter is **optional**.
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
    /// This parameter is **optional**.
    #[must_use]
    pub fn error_handler<F: Into<ErrorHandler>>(self, handler: F) -> Self {
        self.prop.set_error_handler(handler);
        self
    }
}

impl<W> WriteSinkBuilder<W, ()>
where
    W: Write + Send,
{
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `target`\n\n\
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
            prop: self.prop,
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
        let logger = build_test_logger(|b| b.sink(sink.clone()).level_filter(LevelFilter::All));

        info!(logger: logger, "hello WriteSink");

        let data = sink.clone_target();
        assert_eq!(data.as_slice(), b"hello WriteSink");
    }
}
