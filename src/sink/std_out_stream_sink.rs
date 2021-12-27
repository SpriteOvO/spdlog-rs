//! Provides a std out stream plain text sink.

use std::io::{self, Write};

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    LevelFilter, Record, Result, StrBuf,
};

/// An enum representing the available standard output streams.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StdOutStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

// `io::stdout()` and `io::stderr()` return different types,
// and `Std***::lock()` is not in any trait, so we need this struct to abstract
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

/// A standard output stream sink.
///
/// For internal use, users should not use it directly.
pub struct StdOutStreamSink {
    level: LevelFilter,
    formatter: Box<dyn Formatter>,
    dest: StdOutStreamDest<io::Stdout, io::Stderr>,
}

impl StdOutStreamSink {
    /// Constructs a [`StdOutStreamSink`].
    ///
    /// Level default maximum (no discard)
    pub fn new(std_out_stream: StdOutStream) -> StdOutStreamSink {
        StdOutStreamSink {
            level: LevelFilter::max(),
            formatter: Box::new(BasicFormatter::new()),
            dest: StdOutStreamDest::new(std_out_stream),
        }
    }
}

impl Sink for StdOutStreamSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut str_buf = StrBuf::new();
        self.formatter.format(record, &mut str_buf)?;

        let mut dest = self.dest.lock();

        writeln!(dest, "{}", str_buf)?;

        // stderr is not buffered, so we don't need to flush it.
        // https://doc.rust-lang.org/std/io/fn.stderr.html
        if let StdOutStreamDest::Stdout(_) = dest {
            dest.flush()?;
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.dest.lock().flush()?;
        Ok(())
    }

    fn level(&self) -> LevelFilter {
        self.level
    }

    fn set_level(&mut self, level: LevelFilter) {
        self.level = level;
    }

    fn formatter(&self) -> &dyn Formatter {
        self.formatter.as_ref()
    }

    fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
    }
}
