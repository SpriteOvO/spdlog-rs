//! Provides a std stream sink.

use std::{
    convert::Infallible,
    io::{self, Write},
};

use crate::{
    formatter::{Formatter, FormatterContext},
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    terminal_style::{LevelStyles, Style, StyleMode},
    Error, ErrorHandler, Level, LevelFilter, Record, Result, StringBuf,
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
    #[must_use]
    fn new(stream: StdStream) -> Self {
        match stream {
            StdStream::Stdout => StdStreamDest::Stdout(io::stdout()),
            StdStream::Stderr => StdStreamDest::Stderr(io::stderr()),
        }
    }

    #[must_use]
    fn lock(&self) -> StdStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>> {
        match self {
            StdStreamDest::Stdout(stream) => StdStreamDest::Stdout(stream.lock()),
            StdStreamDest::Stderr(stream) => StdStreamDest::Stderr(stream.lock()),
        }
    }

    fn stream_type(&self) -> StdStream {
        match self {
            StdStreamDest::Stdout(_) => StdStream::Stdout,
            StdStreamDest::Stderr(_) => StdStream::Stderr,
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
/// It writes styled text or plain text according to the given [`StyleMode`] and
/// the current terminal environment.
///
/// Note that this sink always flushes the buffer once with each logging.
pub struct StdStreamSink {
    prop: SinkProp,
    dest: StdStreamDest<io::Stdout, io::Stderr>,
    should_render_style: bool,
    level_styles: LevelStyles,
}

impl StdStreamSink {
    /// Gets a builder of `StdStreamSink` with default parameters:
    ///
    /// | Parameter         | Default Value               |
    /// |-------------------|-----------------------------|
    /// | [level_filter]    | `All`                       |
    /// | [formatter]       | `FullFormatter`             |
    /// | [error_handler]   | [`ErrorHandler::default()`] |
    /// |                   |                             |
    /// | [std_stream]      | *must be specified*         |
    /// | [style_mode]      | `Auto`                      |
    ///
    /// [level_filter]: StdStreamSinkBuilder::level_filter
    /// [formatter]: StdStreamSinkBuilder::formatter
    /// [error_handler]: StdStreamSinkBuilder::error_handler
    /// [`ErrorHandler::default()`]: crate::error::ErrorHandler::default()
    /// [std_stream]: StdStreamSinkBuilder::std_stream
    /// [style_mode]: StdStreamSinkBuilder::style_mode
    #[must_use]
    pub fn builder() -> StdStreamSinkBuilder<()> {
        StdStreamSinkBuilder {
            prop: SinkProp::default(),
            std_stream: (),
            style_mode: StyleMode::Auto,
        }
    }

    /// Constructs a `StdStreamSink`.
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `StdStreamSink::builder()` instead"
    )]
    #[must_use]
    pub fn new(std_stream: StdStream, style_mode: StyleMode) -> StdStreamSink {
        Self::builder()
            .std_stream(std_stream)
            .style_mode(style_mode)
            .build()
            .unwrap()
    }

    /// Sets the style of the specified log level.
    pub fn set_style(&mut self, level: Level, style: Style) {
        self.level_styles.set_style(level, style);
    }

    /// Sets the style mode.
    pub fn set_style_mode(&mut self, style_mode: StyleMode) {
        self.should_render_style = Self::should_render_style(style_mode, self.dest.stream_type());
    }

    #[must_use]
    fn should_render_style(style_mode: StyleMode, stream: StdStream) -> bool {
        use is_terminal::IsTerminal;
        let is_terminal = match stream {
            StdStream::Stdout => io::stdout().is_terminal(),
            StdStream::Stderr => io::stderr().is_terminal(),
        };

        match style_mode {
            StyleMode::Always => true,
            StyleMode::Auto => is_terminal && enable_ansi_escape_sequences(),
            StyleMode::Never => false,
        }
    }
}

impl GetSinkProp for StdStreamSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for StdStreamSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        let mut dest = self.dest.lock();

        (|| {
            // TODO: Simplify the if block when our MSRV reaches let-chain support.
            if self.should_render_style {
                if let Some(style_range) = ctx.style_range() {
                    let style = self.level_styles.style(record.level());

                    dest.write_all(string_buf[..style_range.start].as_bytes())?;
                    style.write_start(&mut dest)?;
                    dest.write_all(string_buf[style_range.start..style_range.end].as_bytes())?;
                    style.write_end(&mut dest)?;
                    dest.write_all(string_buf[style_range.end..].as_bytes())?;
                } else {
                    dest.write_all(string_buf.as_bytes())?;
                }
            } else {
                dest.write_all(string_buf.as_bytes())?;
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
}

// --------------------------------------------------

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct StdStreamSinkBuilder<ArgSS> {
    prop: SinkProp,
    std_stream: ArgSS,
    style_mode: StyleMode,
}

impl<ArgSS> StdStreamSinkBuilder<ArgSS> {
    /// Specifies the target standard stream as stdout.
    ///
    /// This is equivalent to `std_stream(StdStream::Stdout)`.
    #[must_use]
    pub fn stdout(self) -> StdStreamSinkBuilder<StdStream> {
        self.std_stream(StdStream::Stdout)
    }

    /// Specifies the target standard stream as stderr.
    ///
    /// This is equivalent to `std_stream(StdStream::Stderr)`.
    #[must_use]
    pub fn stderr(self) -> StdStreamSinkBuilder<StdStream> {
        self.std_stream(StdStream::Stderr)
    }

    /// Specifies the target standard stream.
    ///
    /// This parameter is **required**.
    #[must_use]
    pub fn std_stream(self, std_stream: StdStream) -> StdStreamSinkBuilder<StdStream> {
        StdStreamSinkBuilder {
            prop: self.prop,
            std_stream,
            style_mode: self.style_mode,
        }
    }

    /// Specifies the style mode.
    ///
    /// This parameter is **optional**.
    #[must_use]
    pub fn style_mode(mut self, style_mode: StyleMode) -> Self {
        self.style_mode = style_mode;
        self
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

impl StdStreamSinkBuilder<()> {
    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `std_stream`\n\n\
    ")]
    pub fn build(self, _: Infallible) {}

    #[doc(hidden)]
    #[deprecated(note = "\n\n\
        builder compile-time error:\n\
        - missing required parameter `std_stream`\n\n\
    ")]
    pub fn build_arc(self, _: Infallible) {}
}

impl StdStreamSinkBuilder<StdStream> {
    /// Builds a [`StdStreamSink`].
    pub fn build(self) -> Result<StdStreamSink> {
        Ok(StdStreamSink {
            prop: self.prop,
            dest: StdStreamDest::new(self.std_stream),
            should_render_style: StdStreamSink::should_render_style(
                self.style_mode,
                self.std_stream,
            ),
            level_styles: LevelStyles::default(),
        })
    }

    /// Builds a `Arc<StdStreamSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<StdStreamSink>> {
        self.build().map(Arc::new)
    }
}

// --------------------------------------------------
#[cfg(windows)]
#[must_use]
fn enable_ansi_escape_sequences() -> bool {
    #[must_use]
    fn enable() -> bool {
        use winapi::um::{
            consoleapi::{GetConsoleMode, SetConsoleMode},
            handleapi::INVALID_HANDLE_VALUE,
            processenv::GetStdHandle,
            winbase::STD_OUTPUT_HANDLE,
            wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        };

        let stdout_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        if stdout_handle == INVALID_HANDLE_VALUE {
            return false;
        }

        let mut original_mode = 0;
        if unsafe { GetConsoleMode(stdout_handle, &mut original_mode) } == 0 {
            return false;
        }

        let new_mode = original_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        (unsafe { SetConsoleMode(stdout_handle, new_mode) }) != 0
    }

    use once_cell::sync::OnceCell;

    static INIT: OnceCell<bool> = OnceCell::new();

    *INIT.get_or_init(enable)
}

#[cfg(not(windows))]
#[must_use]
fn enable_ansi_escape_sequences() -> bool {
    true
}
