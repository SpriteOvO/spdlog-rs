//! Provides a std stream sink.

use std::{
    convert::Infallible,
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
    /// Constructs a builder of `StdStreamSink`.
    pub fn builder() -> StdStreamSinkBuilder<()> {
        StdStreamSinkBuilder {
            std_stream: (),
            style_mode: StyleMode::Auto,
        }
    }

    /// Constructs a `StdStreamSink`.
    #[deprecated(note = "it may be removed in the future, use `StdStreamSink::builder()` instead")]
    pub fn new(std_stream: StdStream, style_mode: StyleMode) -> StdStreamSink {
        Self::builder()
            .std_stream(std_stream)
            .style_mode(style_mode)
            .build()
            .unwrap()
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

// --------------------------------------------------

/// The builder of [`StdStreamSink`].
#[doc = include_str!("../include/doc/generic-builder-note.md")]
///
/// # Examples
///
/// - Building a [`StdStreamSink`].
///
///   ```
///   use spdlog::{
///       sink::{StdStreamSink, StdStream},
///       terminal_style::StyleMode
///   };
///
///   let sink: spdlog::Result<StdStreamSink> = StdStreamSink::builder()
///       .std_stream(StdStream::Stdout) // required
///       /* .style_mode(StyleMode::Never) // optional, defaults to
///                                        // `StyleMode::Auto` */
///       .build();
///   ```
///
/// - If any required parameters are missing, a compile-time error will be
///   raised.
///
///   ```compile_fail
///   use spdlog::{
///       sink::{StdStreamSink, StdStream},
///       terminal_style::StyleMode
///   };
///
///   let sink: spdlog::Result<StdStreamSink> = StdStreamSink::builder()
///       // .std_stream(StdStream::Stdout) // required
///       .style_mode(StyleMode::Never) /* optional, defaults to
///                                      * `StyleMode::Auto` */
///       .build();
///   ```
pub struct StdStreamSinkBuilder<ArgSS> {
    std_stream: ArgSS,
    style_mode: StyleMode,
}

impl<ArgSS> StdStreamSinkBuilder<ArgSS> {
    /// Specifies the target standard stream.
    ///
    /// This parameter is required.
    pub fn std_stream(self, std_stream: StdStream) -> StdStreamSinkBuilder<StdStream> {
        StdStreamSinkBuilder {
            std_stream,
            style_mode: self.style_mode,
        }
    }

    /// Specifies the style mode.
    ///
    /// This parameter is optional, and defaults to [`StyleMode::Auto`].
    pub fn style_mode(self, style_mode: StyleMode) -> Self {
        Self { style_mode, ..self }
    }
}

impl StdStreamSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required field `std_stream`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}
}

impl StdStreamSinkBuilder<StdStream> {
    /// Builds a [`StdStreamSink`].
    pub fn build(self) -> Result<StdStreamSink> {
        let atty_stream = match self.std_stream {
            StdStream::Stdout => atty::Stream::Stdout,
            StdStream::Stderr => atty::Stream::Stderr,
        };

        Ok(StdStreamSink {
            level_filter: Atomic::new(LevelFilter::All),
            formatter: SpinRwLock::new(Box::new(FullFormatter::new())),
            dest: StdStreamDest::new(self.std_stream),
            atty_stream,
            should_render_style: StdStreamSink::should_render_style(self.style_mode, atty_stream),
            level_style_codes: LevelStyleCodes::default(),
        })
    }
}

// --------------------------------------------------
#[cfg(windows)]
fn enable_ansi_escape_sequences() -> bool {
    crossterm::ansi_support::supports_ansi()
}

#[cfg(not(windows))]
fn enable_ansi_escape_sequences() -> bool {
    true
}
