//! Provides a std out stream plain text sink.

use std::{
    io::{self, Write},
    mem,
    sync::atomic::Ordering,
};

use atomic::Atomic;

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    Error, LevelFilter, Record, Result, StringBuf,
};

/// An enum representing the available standard output streams.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StdOutStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

// `io::stdout()` and `io::stderr()` return different types, and
// `Std***::lock()` is not in any trait, so we need this struct to abstract
// them.
#[derive(Debug)]
pub(crate) enum StdOutStreamDest<O, E> {
    Stdout(O),
    Stderr(E),
}

impl StdOutStreamDest<io::Stdout, io::Stderr> {
    pub(crate) fn new(stream: StdOutStream) -> Self {
        match stream {
            StdOutStream::Stdout => StdOutStreamDest::Stdout(io::stdout()),
            StdOutStream::Stderr => StdOutStreamDest::Stderr(io::stderr()),
        }
    }

    pub(crate) fn lock(&self) -> StdOutStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>> {
        match self {
            StdOutStreamDest::Stdout(stream) => StdOutStreamDest::Stdout(stream.lock()),
            StdOutStreamDest::Stderr(stream) => StdOutStreamDest::Stderr(stream.lock()),
        }
    }
}

macro_rules! impl_write_for_dest {
    ( $dest:ty ) => {
        impl Write for $dest {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                match self {
                    StdOutStreamDest::Stdout(stream) => stream.write(buf),
                    StdOutStreamDest::Stderr(stream) => stream.write(buf),
                }
            }

            fn flush(&mut self) -> io::Result<()> {
                match self {
                    StdOutStreamDest::Stdout(stream) => stream.flush(),
                    StdOutStreamDest::Stderr(stream) => stream.flush(),
                }
            }
        }
    };
}
impl_write_for_dest!(StdOutStreamDest<io::Stdout, io::Stderr>);
impl_write_for_dest!(StdOutStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>>);

/// A sink with a std output stream as the target.
///
/// It writes plain text.
///
/// Note that this sink always flushes the buffer once with each logging.
pub struct StdOutStreamSink {
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    dest: StdOutStreamDest<io::Stdout, io::Stderr>,
}

impl StdOutStreamSink {
    /// Constructs a `StdOutStreamSink`.
    pub fn new(std_out_stream: StdOutStream) -> StdOutStreamSink {
        StdOutStreamSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            dest: StdOutStreamDest::new(std_out_stream),
        }
    }
}

impl Sink for StdOutStreamSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        self.formatter.read().format(record, &mut string_buf)?;

        let mut dest = self.dest.lock();

        dest.write_all(string_buf.as_bytes())
            .map_err(Error::WriteRecord)?;

        // stderr is not buffered, so we don't need to flush it.
        // https://doc.rust-lang.org/std/io/fn.stderr.html
        if let StdOutStreamDest::Stdout(_) = dest {
            dest.flush().map_err(Error::FlushBuffer)?;
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.dest.lock().flush().map_err(Error::FlushBuffer)
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
