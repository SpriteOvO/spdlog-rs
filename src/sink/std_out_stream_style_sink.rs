//! Provides a std out stream style text sink.

pub use crate::sink::std_out_stream_sink::StdOutStream;

use std::io::{self, Write};

use if_chain::if_chain;

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::{std_out_stream_sink::StdOutStreamDest, Sink},
    terminal::{LevelStyleCodes, Style, StyleMode},
    LevelFilter, Record, Result, StringBuf,
};

/// A standard output stream style sink.
///
/// For internal use, users should not use it directly.
pub struct StdOutStreamStyleSink {
    level: LevelFilter,
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
            level: LevelFilter::max(),
            formatter: Box::new(BasicFormatter::new()),
            dest: StdOutStreamDest::new(std_out_stream),
            atty_stream,
            should_render_style: Self::should_render_style(style_mode, atty_stream),
            level_style_codes: LevelStyleCodes::default(),
        }
    }

    /// Sets the style of the specified log level.
    pub fn set_style(&mut self, level: LevelFilter, style: Style) {
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

        if_chain! {
            if self.should_render_style;
            if let Some(style_range) = extra_info.style_range();
            then {
                let style_code = self.level_style_codes.code(record.level().to_level_filter());

                dest.write_all(string_buf[..style_range.start].as_bytes())?;
                dest.write_all(style_code.start.as_bytes())?;
                dest.write_all(string_buf[style_range.start..style_range.end].as_bytes())?;
                dest.write_all(style_code.end.as_bytes())?;
                writeln!(dest, "{}", &string_buf[style_range.end..])?;
            } else {
                writeln!(dest, "{}", string_buf)?;
            }
        }

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
