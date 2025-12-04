//! Provides a std stream sink.

use std::{
    convert::Infallible,
    io::{self, Write},
    // Import `str` module for function `std::str::from_utf8`, because method `str::from_utf8` is
    // stabilized since Rust 1.87.
    //
    // TODO: Remove this import when our MSRV reaches Rust 1.87.
    str,
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

impl StdStream {
    fn via_write(&self) -> StdStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>> {
        match self {
            Self::Stdout => StdStreamDest::Stdout(io::stdout().lock()),
            Self::Stderr => StdStreamDest::Stderr(io::stderr().lock()),
        }
    }

    fn via_macro(&self) -> StdStreamDest<via_macro::Stdout, via_macro::Stderr> {
        match self {
            Self::Stdout => StdStreamDest::Stdout(via_macro::Stdout),
            Self::Stderr => StdStreamDest::Stderr(via_macro::Stderr),
        }
    }
}

// `io::stdout()` and `io::stderr()` return different types, and
// `Std***::lock()` is not in any trait, so we need this struct to abstract
// them.
#[derive(Debug)]
enum StdStreamDest<O, E> {
    Stdout(O),
    Stderr(E),
}

impl<O, E> StdStreamDest<O, E> {
    #[expect(dead_code)]
    fn stream_type(&self) -> StdStream {
        match self {
            Self::Stdout(_) => StdStream::Stdout,
            Self::Stderr(_) => StdStream::Stderr,
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
impl_write_for_dest!(StdStreamDest<io::StdoutLock<'_>, io::StderrLock<'_>>);
impl_write_for_dest!(StdStreamDest<via_macro::Stdout, via_macro::Stderr>);

mod via_macro {
    use super::*;

    fn bytes_to_str(buf: &[u8]) -> io::Result<&str> {
        str::from_utf8(buf).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    pub(crate) struct Stdout;

    impl Write for Stdout {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            print!("{}", bytes_to_str(buf)?);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            io::stdout().flush()
        }
    }

    pub(crate) struct Stderr;

    impl Write for Stderr {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            eprint!("{}", bytes_to_str(buf)?);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            io::stderr().flush()
        }
    }
}

/// A sink with a std stream as the target.
///
/// It writes styled text or plain text according to the given [`StyleMode`] and
/// the current terminal environment.
///
/// Note that this sink always flushes the buffer once with each logging.
pub struct StdStreamSink {
    prop: SinkProp,
    via_print_macro: bool,
    std_stream: StdStream,
    should_render_style: bool,
    level_styles: LevelStyles,
}

impl StdStreamSink {
    /// Gets a builder of `StdStreamSink` with default parameters:
    ///
    /// | Parameter         | Default Value                                                       |
    /// |-------------------|---------------------------------------------------------------------|
    /// | [level_filter]    | [`LevelFilter::All`]                                                |
    /// | [formatter]       | [`FullFormatter`]                                                   |
    /// | [error_handler]   | [`ErrorHandler::default()`]                                         |
    /// |                   |                                                                     |
    /// | [std_stream]      | *must be specified*                                                 |
    /// | [style_mode]      | [`StyleMode::Auto`]                                                 |
    /// | [via_print_macro] | `false`, or `true` if feature gate `std-stream-captured` is enabled |
    ///
    /// [level_filter]: StdStreamSinkBuilder::level_filter
    /// [formatter]: StdStreamSinkBuilder::formatter
    /// [`FullFormatter`]: crate::formatter::FullFormatter
    /// [error_handler]: StdStreamSinkBuilder::error_handler
    /// [std_stream]: StdStreamSinkBuilder::std_stream
    /// [style_mode]: StdStreamSinkBuilder::style_mode
    /// [via_print_macro]: StdStreamSinkBuilder::via_print_macro
    #[must_use]
    pub fn builder() -> StdStreamSinkBuilder<()> {
        StdStreamSinkBuilder {
            prop: SinkProp::default(),
            std_stream: (),
            style_mode: StyleMode::Auto,
            via_print_macro: cfg!(feature = "std-stream-captured"),
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
        self.should_render_style = Self::should_render_style(style_mode, self.std_stream);
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

        if !self.via_print_macro {
            self.log_write(record, &string_buf, &ctx, self.std_stream.via_write())
        } else {
            self.log_write(record, &string_buf, &ctx, self.std_stream.via_macro())
        }
    }

    fn flush(&self) -> Result<()> {
        if !self.via_print_macro {
            self.std_stream.via_write().flush()
        } else {
            self.std_stream.via_macro().flush()
        }
        .map_err(Error::FlushBuffer)
    }
}

impl StdStreamSink {
    fn log_write<O: Write, E: Write>(
        &self,
        record: &Record,
        string_buf: &StringBuf,
        ctx: &FormatterContext<'_>,
        mut dest: StdStreamDest<O, E>,
    ) -> Result<()>
    where
        StdStreamDest<O, E>: Write,
    {
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
        if let StdStream::Stdout = self.std_stream {
            dest.flush().map_err(Error::FlushBuffer)?;
        }

        Ok(())
    }
}

// --------------------------------------------------

/// #
#[doc = include_str!("../include/doc/generic-builder-note.md")]
pub struct StdStreamSinkBuilder<ArgSS> {
    prop: SinkProp,
    std_stream: ArgSS,
    style_mode: StyleMode,
    via_print_macro: bool,
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
            via_print_macro: self.via_print_macro,
        }
    }

    /// Specifies the style mode.
    ///
    /// This parameter is **optional**, and defaults to [StyleMode::Auto].
    #[must_use]
    pub fn style_mode(mut self, style_mode: StyleMode) -> Self {
        self.style_mode = style_mode;
        self
    }

    /// Specifies to use `print!` and `eprint!` macros for output.
    ///
    /// If enabled, the sink will use [`print!`] and [`eprint!`] macros instead
    /// of [`io::Write`] trait with [`io::stdout`] and [`io::stderr`] to output
    /// logs. This is useful if you want the logs to be [captured] by `cargo
    /// test` and `cargo bench`.
    ///
    /// This parameter is **optional**, and defaults to `false`, or defaults to
    /// `true` if feature gate `std-stream-captured` is enabled.
    ///
    /// A convienient way to enable it for `cargo test` and `cargo bench` is to
    /// add the following lines to your `Cargo.toml`:
    ///
    /// ```toml
    /// # Note that it's not [dependencies]
    ///
    /// [dev-dependencies]
    /// spdlog-rs = { version = "...", features = ["std-stream-captured"] }
    /// ```
    ///
    /// [captured]: https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
    #[must_use]
    pub fn via_print_macro(mut self) -> Self {
        self.via_print_macro = true;
        self
    }

    // Prop
    //

    /// Specifies a log level filter.
    ///
    /// This parameter is **optional**, and defaults to [`LevelFilter::All`].
    #[must_use]
    pub fn level_filter(self, level_filter: LevelFilter) -> Self {
        self.prop.set_level_filter(level_filter);
        self
    }

    /// Specifies a formatter.
    ///
    /// This parameter is **optional**, and defaults to [`FullFormatter`].
    ///
    /// [`FullFormatter`]: crate::formatter::FullFormatter
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
    /// This parameter is **optional**, and defaults to
    /// [`ErrorHandler::default()`].
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
            via_print_macro: self.via_print_macro,
            std_stream: self.std_stream,
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
