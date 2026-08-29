use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{
    formatter::{Formatter, FormatterContext, FullFormatter},
    prelude::*,
    sink::{GetSinkProp, Sink, SinkProp},
    sync::*,
    Error, ErrorHandler, Record, Result, StringBuf,
};

/// A sink with [Web console API] as the target.
///
/// # Log Level Mapping
///
/// | spdlog-rs  | console API     |
/// |------------|-----------------|
/// | `Critical` | `console.error` |
/// | `Error`    | `console.error` |
/// | `Warn`     | `console.warn`  |
/// | `Info`     | `console.info`  |
/// | `Debug`    | `console.debug` |
/// | `Trace`    | `console.trace` |
///
/// [Web console API]: https://developer.mozilla.org/en-US/docs/Web/API/console
pub struct WebConsoleSink {
    prop: SinkProp,
}

impl WebConsoleSink {
    /// Gets a builder of `WebConsoleSink` with default parameters:
    ///
    /// | Parameter       | Default Value                           |
    /// |-----------------|-----------------------------------------|
    /// | [level_filter]  | [`LevelFilter::All`]                    |
    /// | [formatter]     | [`FullFormatter`] `(!time !level !eol)` |
    /// | [error_handler] | [`ErrorHandler::default()`]             |
    ///
    /// [level_filter]: WebConsoleSinkBuilder::level_filter
    /// [formatter]: WebConsoleSinkBuilder::formatter
    /// [`FullFormatter`]: crate::formatter::FullFormatter
    /// [error_handler]: WebConsoleSinkBuilder::error_handler
    #[must_use]
    pub fn builder() -> WebConsoleSinkBuilder {
        let prop = SinkProp::default();
        prop.set_formatter(
            FullFormatter::builder()
                .time(false)
                .level(false)
                .eol(false)
                .build(),
        );

        WebConsoleSinkBuilder { prop }
    }
}

impl GetSinkProp for WebConsoleSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for WebConsoleSink {
    fn log(&self, record: &Record) -> Result<()> {
        let mut string_buf = StringBuf::new();
        let mut ctx = FormatterContext::new();
        self.prop
            .formatter()
            .format(record, &mut string_buf, &mut ctx)?;

        let text = JsValue::from_str(&string_buf);
        match record.level() {
            Level::Trace => console::trace_1(&text),
            Level::Debug => console::debug_1(&text),
            Level::Info => console::info_1(&text),
            Level::Warn => console::warn_1(&text),
            Level::Error | Level::Critical => console::error_1(&text),
        }
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

#[allow(missing_docs)]
pub struct WebConsoleSinkBuilder {
    prop: SinkProp,
}

impl WebConsoleSinkBuilder {
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

    //

    /// Builds a [`WebConsoleSink`].
    pub fn build(self) -> Result<WebConsoleSink> {
        let sink = WebConsoleSink { prop: self.prop };
        Ok(sink)
    }

    /// Builds a `Arc<WebConsoleSink>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    pub fn build_arc(self) -> Result<Arc<WebConsoleSink>> {
        self.build().map(Arc::new)
    }
}
