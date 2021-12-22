use std::{io::Write, sync::Mutex};

use if_chain::if_chain;

use crate::{
    formatter::{BasicFormatter, Formatter},
    sink::Sink,
    terminal::{LevelStyleCodes, Style, StyleMode},
    LevelFilter, LogMsg, Result, StrBuf,
};

/// A standard output stream style sink.
///
/// For internal use, users should not use it directly.
pub struct StdOutStreamStyleSink<S>
where
    S: Write + Send + Sync,
{
    level: LevelFilter,
    formatter: Box<dyn Formatter>,
    out_stream: Mutex<S>,
    atty_stream: atty::Stream,
    should_render_style: bool,
    level_style_codes: LevelStyleCodes,
}

impl<S> StdOutStreamStyleSink<S>
where
    S: Write + Send + Sync,
{
    /// Constructs a [`StdOutStreamStyleSink`].
    ///
    /// Level default maximum (no discard)
    pub fn new(out_stream: S, atty_stream: atty::Stream) -> StdOutStreamStyleSink<S> {
        StdOutStreamStyleSink::with_style_mode(out_stream, atty_stream, StyleMode::Auto)
    }

    /// Constructs a [`StdOutStreamStyleSink`] with a style mode.
    ///
    /// Level default maximum (no discard)
    pub fn with_style_mode(
        out_stream: S,
        atty_stream: atty::Stream,
        style_mode: StyleMode,
    ) -> StdOutStreamStyleSink<S> {
        StdOutStreamStyleSink {
            level: LevelFilter::max(),
            formatter: Box::new(BasicFormatter::new()),
            out_stream: Mutex::new(out_stream),
            atty_stream,
            should_render_style: Self::should_render_style(style_mode, atty_stream),
            level_style_codes: LevelStyleCodes::default(),
        }
    }

    fn should_render_style(style_mode: StyleMode, atty_stream: atty::Stream) -> bool {
        match style_mode {
            StyleMode::Always => true,
            StyleMode::Auto => atty::is(atty_stream),
            StyleMode::Never => false,
        }
    }
}

impl<S> Sink for StdOutStreamStyleSink<S>
where
    S: Write + Send + Sync,
{
    fn log(&self, msg: &LogMsg) -> Result<()> {
        let mut str_buf = StrBuf::new();

        let extra_info = self.formatter.format(msg, &mut str_buf)?;

        let mut out_stream = self.out_stream.lock().unwrap();

        if_chain! {
            if self.should_render_style;
            if let Some(style_range) = extra_info.style_range();
            then {
                let style_code = self.level_style_codes.code(msg.level().to_level_filter());

                out_stream.write_all(str_buf[..style_range.start].as_bytes())?;
                out_stream.write_all(style_code.start.as_bytes())?;
                out_stream.write_all(str_buf[style_range.start..style_range.end].as_bytes())?;
                out_stream.write_all(style_code.end.as_bytes())?;
                writeln!(out_stream, "{}", &str_buf[style_range.end..])?;
            } else {
                writeln!(out_stream, "{}", str_buf)?;
            }
        }

        out_stream.flush()?;

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.out_stream.lock().unwrap().flush()?;
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

/// A trait for style sinks.
pub trait StyleSink: Sink {
    /// Sets the style of the specified log level.
    fn set_style(&mut self, level: LevelFilter, style: Style);

    /// Sets the style mode.
    fn set_style_mode(&mut self, mode: StyleMode);
}

impl<S> StyleSink for StdOutStreamStyleSink<S>
where
    S: Write + Send + Sync,
{
    fn set_style(&mut self, level: LevelFilter, style: Style) {
        self.level_style_codes.set_code(level, style);
    }

    fn set_style_mode(&mut self, style_mode: StyleMode) {
        self.should_render_style = Self::should_render_style(style_mode, self.atty_stream);
    }
}

pub(crate) mod macros {
    macro_rules! forward_style_sink_methods {
        ($struct_type:ident, $inner_name:ident) => {
            use crate::{
                sink::{macros::forward_sink_methods, StyleSink},
                terminal::{Style, StyleMode},
            };

            forward_sink_methods!($struct_type, $inner_name);

            impl StyleSink for $struct_type {
                fn set_style(&mut self, level: LevelFilter, style: Style) {
                    self.$inner_name.set_style(level, style);
                }

                fn set_style_mode(&mut self, mode: StyleMode) {
                    self.$inner_name.set_style_mode(mode);
                }
            }
        };
    }
    pub(crate) use forward_style_sink_methods;
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use log::RecordBuilder;

    use super::*;
    use crate::{
        terminal::{Color, StyleBuilder, StyleCode},
        Level,
    };

    #[test]
    fn log() {
        let mut out_stream = Vec::<u8>::new();

        let mut sink = StdOutStreamStyleSink::new(&mut out_stream, atty::Stream::Stdout);

        sink.set_style_mode(StyleMode::Always);

        let record = RecordBuilder::new()
            .level(Level::Warn)
            .target("target")
            .args(format_args!("test log content"))
            .build();

        let msg = LogMsg::new(&record);

        sink.log(&msg).unwrap();

        let style_code: StyleCode = StyleBuilder::new()
            .color(Color::Yellow)
            .bold()
            .build()
            .into();

        assert_eq!(
            format!(
                "[{}] [target] [{}WARN{}] test log content\n",
                Into::<DateTime::<Local>>::into(msg.time().clone()).format("%Y-%m-%d %H:%M:%S.%3f"),
                style_code.start,
                style_code.end
            )
            .as_bytes(),
            out_stream
        );
    }

    #[test]
    fn style() {
        let mut out_stream = Vec::<u8>::new();

        let mut sink = StdOutStreamStyleSink::new(&mut out_stream, atty::Stream::Stdout);

        sink.set_style_mode(StyleMode::Always);

        let record = RecordBuilder::new()
            .level(Level::Error)
            .target("target")
            .args(format_args!("test log content"))
            .build();

        let msg = LogMsg::new(&record);

        // log with the default style
        sink.log(&msg).unwrap();

        // change the style, log again
        sink.set_style(
            LevelFilter::Error,
            StyleBuilder::new().color(Color::Cyan).build(),
        );
        sink.log(&msg).unwrap();

        let before_style_code: StyleCode =
            StyleBuilder::new().color(Color::Red).bold().build().into();

        let now_style_code: StyleCode = StyleBuilder::new().color(Color::Cyan).build().into();

        assert_eq!(
            format!(
                "[{}] [target] [{}ERROR{}] test log content\n\
                 [{}] [target] [{}ERROR{}] test log content\n",
                Into::<DateTime::<Local>>::into(msg.time().clone()).format("%Y-%m-%d %H:%M:%S.%3f"),
                before_style_code.start,
                before_style_code.end,
                Into::<DateTime::<Local>>::into(msg.time().clone()).format("%Y-%m-%d %H:%M:%S.%3f"),
                now_style_code.start,
                now_style_code.end
            )
            .as_bytes(),
            out_stream
        );
    }
}
