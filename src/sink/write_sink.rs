use std::{
    io::Write,
    mem,
    sync::{self, atomic::Ordering},
};

use atomic::Atomic;

use crate::{
    formatter::{Formatter, FullFormatter},
    prelude::*,
    Error, Record, Result, Sink, StringBuf,
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
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    target: sync::Mutex<W>,
}

impl<W> WriteSink<W>
where
    W: Write + Send,
{
    /// Constructs a `WriteSink` that writes log messages into the given `impl
    /// Write` object.
    pub fn new(target: W) -> Self {
        Self {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            target: sync::Mutex::new(target),
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

    fn lock_target(&self) -> sync::MutexGuard<W> {
        const MUTEX_POISONED_MESSAGE: &str = "mutex is poisoned";
        self.target.lock().expect(MUTEX_POISONED_MESSAGE)
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
        self.formatter.read().format(record, &mut string_buf)?;

        self.lock_target()
            .write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.lock_target().flush().map_err(Error::FlushBuffer)
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

impl<W> Drop for WriteSink<W>
where
    W: Write + Send,
{
    fn drop(&mut self) {
        let flush_result = self.lock_target().flush().map_err(Error::FlushBuffer);
        if let Err(err) = flush_result {
            // Sinks do not have an error handler, because it would increase complexity and
            // the error is not common. So currently users cannot handle this error by
            // themselves.
            crate::default_error_handler("WriteSink", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::sync::Arc;

    #[test]
    fn validation() {
        let sink = Arc::new(WriteSink::new(Vec::new()));
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
