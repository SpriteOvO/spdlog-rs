//! Provides a std out stream style text sink.

pub use crate::sink::std_out_stream_sink::StdOutStream;

use std::io::{self, Write};

use if_chain::if_chain;

use crate::{
    formatter::{Formatter, FullFormatter},
    sink::{std_out_stream_sink::StdOutStreamDest, Sink},
    terminal_style::{LevelStyleCodes, Style, StyleMode},
    Error, Level, LevelFilter, Record, Result, StringBuf,
};

/// A standard output stream style sink.
///
/// For internal use, users should not use it directly.
pub struct StdOutStreamStyleSink {
    level_filter: LevelFilter,
    formatter: Box<dyn Formatter>,
    dest: StdOutStreamDest<io::Stdout, io::Stderr>,
    atty_stream: atty::Stream,
    should_render_style: bool,
    level_style_codes: LevelStyleCodes,
}

impl StdOutStreamStyleSink {
    /// Constructs a [`StdOutStreamStyleSink`].
    ///
    /// Level default maximum (no discard)
    pub fn new(std_out_stream: StdOutStream, style_mode: StyleMode) -> StdOutStreamStyleSink {
        let atty_stream = match std_out_stream {
            StdOutStream::Stdout => atty::Stream::Stdout,
            StdOutStream::Stderr => atty::Stream::Stderr,
        };

        StdOutStreamStyleSink {
            level_filter: LevelFilter::All,
            formatter: Box::new(FullFormatter::new()),
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

impl Sink for StdOutStreamStyleSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();

        let extra_info = self.formatter.format(record, &mut string_buf)?;

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
        self.level_filter
    }

    fn set_level_filter(&mut self, level_filter: LevelFilter) {
        self.level_filter = level_filter;
    }

    fn formatter(&self) -> &dyn Formatter {
        self.formatter.as_ref()
    }

    fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
    }
}
