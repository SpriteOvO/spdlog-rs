//! Provides a std stream sink.

use std::{
    io::{self, Write},
    mem,
};

use if_chain::if_chain;

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::Sink,
    sync::*,
    terminal_style::{LevelStyleCodes, Style, StyleMode},
    Error, Level, LevelFilter, Record, Result, StringBuf,
};

/// An enum representing the available standard streams.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StdStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

// `io::stdout()` and `io::stderr()` return different types, and
// `Std***::lock()` is not in any trait, so we need this struct to abstract
// them.
#[derive(Debug)]
enum StdStreamDest<O, E> {
    Stdout(O),
    Stderr(E),
}

impl StdStreamDest<io::Stdout, io::Stderr> {
    fn new(stream: StdStream) -> Self {
        match stream {
            StdStream::Stdout => StdStreamDest::Stdout(io::stdout()),
            StdStream::Stderr => StdStreamDest::Stderr(io::stderr()),
        }
    }

    fn lock(&self) -> StdStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>> {
        match self {
            StdStreamDest::Stdout(stream) => StdStreamDest::Stdout(stream.lock()),
            StdStreamDest::Stderr(stream) => StdStreamDest::Stderr(stream.lock()),
        }
    }
}

macro_rules! impl_write_for_dest {
    ( $dest:ty ) => {
        impl Write for $dest {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                match self {
                    StdStreamDest::Stdout(stream) => stream.write(buf),
                    StdStreamDest::Stderr(stream) => stream.write(buf),
                }
            }

            fn flush(&mut self) -> io::Result<()> {
                match self {
                    StdStreamDest::Stdout(stream) => stream.flush(),
                    StdStreamDest::Stderr(stream) => stream.flush(),
                }
            }
        }
    };
}
impl_write_for_dest!(StdStreamDest<io::Stdout, io::Stderr>);
impl_write_for_dest!(StdStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>>);

/// A sink with a std stream as the target.
///
/// It writes styled text or plain text according to the given [`StyleMode`].
///
/// Note that this sink always flushes the buffer once with each logging.
pub struct StdStreamSink {
    level_filter: Atomic<LevelFilter>,
    formatter: SpinRwLock<Box<dyn Formatter>>,
    dest: StdStreamDest<io::Stdout, io::Stderr>,
    atty_stream: atty::Stream,
    should_render_style: bool,
    level_style_codes: LevelStyleCodes,
}

impl StdStreamSink {
    /// Constructs a `StdStreamSink`.
    pub fn new(std_stream: StdStream, style_mode: StyleMode) -> StdStreamSink {
        let atty_stream = match std_stream {
            StdStream::Stdout => atty::Stream::Stdout,
            StdStream::Stderr => atty::Stream::Stderr,
        };

        StdStreamSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            dest: StdStreamDest::new(std_stream),
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
            StyleMode::Auto => atty::is(atty_stream) && enable_ansi_escape_sequences(),
            StyleMode::Never => false,
        }
    }
}

impl Sink for StdStreamSink {
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
        if let StdStreamDest::Stdout(_) = dest {
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

#[cfg(windows)]
fn enable_ansi_escape_sequences() -> bool {
    crossterm::ansi_support::supports_ansi()
}

#[cfg(not(windows))]
fn enable_ansi_escape_sequences() -> bool {
    true
}
