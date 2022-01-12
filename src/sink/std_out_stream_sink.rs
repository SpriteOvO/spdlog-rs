//! Provides a std out stream sink.

use std::{
    io::{self, Write},
    mem,
    sync::atomic::Ordering,
};

use atomic::Atomic;
use if_chain::if_chain;

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    terminal_style::{LevelStyleCodes, Style, StyleMode},
    Error, Level, LevelFilter, Record, Result, StringBuf,
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
enum StdOutStreamDest<O, E> {
    Stdout(O),
    Stderr(E),
}

impl StdOutStreamDest<io::Stdout, io::Stderr> {
    fn new(stream: StdOutStream) -> Self {
        match stream {
            StdOutStream::Stdout => StdOutStreamDest::Stdout(io::stdout()),
            StdOutStream::Stderr => StdOutStreamDest::Stderr(io::stderr()),
        }
    }

    fn lock(&self) -> StdOutStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>> {
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
/// It writes styled text or plain text according to the given [`StyleMode`].
///
/// Note that this sink always flushes the buffer once with each logging.
pub struct StdOutStreamSink {
    level_filter: Atomic<LevelFilter>,
    formatter: spin::RwLock<Box<dyn Formatter>>,
    dest: StdOutStreamDest<io::Stdout, io::Stderr>,
    atty_stream: atty::Stream,
    should_render_style: bool,
    level_style_codes: LevelStyleCodes,
}

impl StdOutStreamSink {
    /// Constructs a `StdOutStreamSink`.
    pub fn new(std_out_stream: StdOutStream, style_mode: StyleMode) -> StdOutStreamSink {
        let atty_stream = match std_out_stream {
            StdOutStream::Stdout => atty::Stream::Stdout,
            StdOutStream::Stderr => atty::Stream::Stderr,
        };

        StdOutStreamSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: spin::RwLock::new(Box::new(FullFormatter::new())),
            dest: StdOutStreamDest::new(std_out_stream),
            atty_stream,
            should_render_style: Self::should_render_style(style_mode, atty_stream),
            level_style_codes: LevelStyleCodes::default(),
        }
    }

    /// Sets the style of the specified log level.
    pub fn set_style(&mut self, level: Level, style: Style) {
        self.level_style_codes.set_code(level, style);
    }

    /// Sets the style mode.
    pub fn set_style_mode(&mut self, style_mode: StyleMode) {
        self.should_render_style = Self::should_render_style(style_mode, self.atty_stream);
    }

    fn should_render_style(style_mode: StyleMode, atty_stream: atty::Stream) -> bool {
        match style_mode {
            StyleMode::Always => true,
            StyleMode::Auto => atty::is(atty_stream),
            StyleMode::Never => false,
        }
    }
}

impl Sink for StdOutStreamSink {
    fn log(&self, record: &Record) -> Result<()> {
        if !self.should_log(record.level()) {
            return Ok(());
        }

        let mut string_buf = StringBuf::new();

        let extra_info = self.formatter.read().format(record, &mut string_buf)?;

        let mut dest = self.dest.lock();

        (|| {
            if_chain! {
                if self.should_render_style;
                if let Some(style_range) = extra_info.style_range();
                then {
                    let style_code = self.level_style_codes.code(record.level());

                    dest.write_all(string_buf[..style_range.start].as_bytes())?;
                    dest.write_all(style_code.start.as_bytes())?;
                    dest.write_all(string_buf[style_range.start..style_range.end].as_bytes())?;
                    dest.write_all(style_code.end.as_bytes())?;
                    dest.write_all(string_buf[style_range.end..].as_bytes())?;
                } else {
                    dest.write_all(string_buf.as_bytes())?;
                }
            }
            Ok(())
        })()
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
