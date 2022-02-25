//! Provides a logger structure.

use std::time::Duration;

use crate::{
    env_level,
    periodic_worker::PeriodicWorker,
    sink::{Sink, Sinks},
    sync::*,
    Error, ErrorHandler, Level, LevelFilter, Record,
};

/// A logger structure.
///
/// A logger contains a combination of sinks, and sinks implement writing log
/// messages to actual targets.
///
/// Users usually log messages through log macros.
///
/// # Examples
///
/// ```
/// # use std::sync::Arc;
/// use std::time::Duration;
/// use spdlog::prelude::*;
///
/// # let custom_logger: Arc<Logger> = spdlog::default_logger();
/// let default_logger: Arc<Logger> = spdlog::default_logger();
/// default_logger.set_level_filter(LevelFilter::All);
/// default_logger.set_flush_period(Some(Duration::from_secs(10)));
/// info!("logging with default logger");
///
/// custom_logger.set_level_filter(LevelFilter::All);
/// custom_logger.set_flush_period(Some(Duration::from_secs(10)));
/// info!(logger: custom_logger, "logging with custom logger");
/// ```
///
/// For more examples, see [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct Logger {
    name: Option<String>,
    level_filter: Atomic<LevelFilter>,
    sinks: Sinks,
    flush_level_filter: Atomic<LevelFilter>,
    periodic_flusher: Mutex<Option<PeriodicWorker>>,
    error_handler: SpinRwLock<Option<ErrorHandler>>,
}

impl Logger {
    /// Constructs a [`LoggerBuilder`].
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Gets the logger name.
    ///
    /// Returns `None` if the logger does not have a name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    /// Determines if a log message with the specified level would be
    /// logged.
    ///
    /// This allows callers to avoid expensive computation of log message
    /// arguments if the message would be discarded anyway.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// use spdlog::prelude::*;
    ///
    /// let logger: Arc<Logger> = spdlog::default_logger();
    ///
    /// logger.set_level_filter(LevelFilter::MoreSevere(Level::Info));
    /// assert_eq!(logger.should_log(Level::Debug), false);
    /// assert_eq!(logger.should_log(Level::Info), false);
    /// assert_eq!(logger.should_log(Level::Warn), true);
    /// assert_eq!(logger.should_log(Level::Error), true);
    ///
    /// logger.set_level_filter(LevelFilter::All);
    /// assert_eq!(logger.should_log(Level::Debug), true);
    /// assert_eq!(logger.should_log(Level::Info), true);
    /// assert_eq!(logger.should_log(Level::Warn), true);
    /// assert_eq!(logger.should_log(Level::Error), true);
    /// ```
    pub fn should_log(&self, level: Level) -> bool {
        self.level_filter().compare(level)
    }

    /// Logs a record.
    ///
    /// Users usually do not use this function directly, use log macros instead.
    pub fn log(&self, record: &Record) {
        if !self.should_log(record.level()) {
            return;
        }
        self.sink_record(record);
    }

    /// Flushes any buffered records.
    ///
    /// Users can call this function to flush manually or use auto-flush
    /// policies. See also [`Logger::flush_level_filter`] and
    /// [`Logger::set_flush_period`].
    ///
    /// Note that it is expensive, calling it frequently will affect
    /// performance.
    pub fn flush(&self) {
        self.flush_sinks();
    }

    /// Gets the flush level filter.
    pub fn flush_level_filter(&self) -> LevelFilter {
        self.flush_level_filter.load(Ordering::Relaxed)
    }

    /// Sets a flush level filter.
    ///
    /// When logging a new record, flush the buffer if this filter condition is
    /// true.
    ///
    /// This auto-flush policy can work with [`Logger::set_flush_period`]
    /// together.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// use spdlog::prelude::*;
    ///
    /// # let logger: Arc<Logger> = spdlog::default_logger();
    /// logger.set_flush_level_filter(LevelFilter::Off);
    /// trace!(logger: logger, "hello");
    /// trace!(logger: logger, "world");
    /// // Until here the buffer may not have been flushed (depending on sinks implementation)
    ///
    /// logger.set_flush_level_filter(LevelFilter::All);
    /// trace!(logger: logger, "hello"); // Logs and flushes the buffer once
    /// trace!(logger: logger, "world"); // Logs and flushes the buffer once
    /// ```
    pub fn set_flush_level_filter(&self, level_filter: LevelFilter) {
        self.flush_level_filter
            .store(level_filter, Ordering::Relaxed);
    }

    /// Gets the log filter level.
    pub fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    /// Sets the log filter level.
    ///
    /// # Examples
    ///
    /// See [`Logger::should_log`].
    pub fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    /// Sets periodic flush.
    ///
    /// This function receives a `&Arc<Self>`. Calling it will spawn a new
    /// thread.
    ///
    /// This auto-flush policy can work with [`Logger::set_flush_level_filter`]
    /// together.
    ///
    /// # Panics
    ///
    ///  - Panics if `interval` is zero.
    ///
    ///  - Panics if this function is called with `Some` value and then clones
    ///    the `Logger` instead of the `Arc<Logger>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// # use std::sync::Arc;
    /// # use spdlog::prelude::*;
    ///
    /// # let logger: Arc<Logger> = spdlog::default_logger();
    /// // From now on, auto-flush the `logger` buffer every 10 seconds.
    /// logger.set_flush_period(Some(Duration::from_secs(10)));
    ///
    /// // Remove periodic auto-flush.
    /// logger.set_flush_period(None);
    /// ```
    pub fn set_flush_period(self: &Arc<Self>, interval: Option<Duration>) {
        let mut periodic_flusher = self.periodic_flusher.lock_expect();

        *periodic_flusher = None;

        if let Some(interval) = interval {
            let weak = Arc::downgrade(self);
            let callback = move || {
                let strong = weak.upgrade();
                if let Some(strong) = strong {
                    strong.flush_sinks();
                    true
                } else {
                    false // All `Arc`s are dropped, return `false` to quit the
                          // worker thread.
                }
            };
            *periodic_flusher = Some(PeriodicWorker::new(callback, interval));
        }
    }

    /// Gets a reference to sinks in the logger.
    pub fn sinks(&self) -> &[Arc<dyn Sink>] {
        &self.sinks
    }

    /// Gets a mutable reference to sinks in the logger.
    pub fn sinks_mut(&mut self) -> &mut Sinks {
        &mut self.sinks
    }

    /// Sets a error handler.
    ///
    /// If an error occurs while logging or flushing, this handler will be
    /// called. If no handler is set, the error will be output to the terminal
    /// and then ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::prelude::*;
    ///
    /// spdlog::default_logger().set_error_handler(Some(|err: spdlog::Error| {
    ///     panic!("spdlog-rs error: {}", err)
    /// }));
    /// ```
    pub fn set_error_handler(&self, handler: Option<ErrorHandler>) {
        *self.error_handler.write() = handler;
    }

    fn sink_record(&self, record: &Record) {
        self.sinks.iter().for_each(|sink| {
            if let Err(err) = sink.log(record) {
                self.handle_error(err);
            }
        });

        if self.should_flush(record) {
            self.flush();
        }
    }

    fn flush_sinks(&self) {
        self.sinks.iter().for_each(|sink| {
            if let Err(err) = sink.flush() {
                self.handle_error(err);
            }
        });
    }

    fn handle_error(&self, err: Error) {
        if let Some(handler) = self.error_handler.read().as_ref() {
            handler(err)
        } else {
            crate::default_error_handler(
                format!(
                    "Logger ({})",
                    self.name.as_ref().map_or("*no name*", String::as_str)
                ),
                err,
            );
        }
    }

    fn should_flush(&self, record: &Record) -> bool {
        self.flush_level_filter().compare(record.level())
    }
}

impl Clone for Logger {
    /// Clones the `Logger`.
    ///
    /// # Panics
    ///
    /// Panics if [`Logger::set_flush_period`] is called with `Some` value and
    /// then clones the `Logger` instead of the `Arc<Logger>`.
    fn clone(&self) -> Self {
        if self.periodic_flusher.lock_expect().is_some() {
            panic!(
                "you can't clone a `Logger` with a `flush_period` value, \
                 clone a `Arc<Logger>` instead."
            );
        }

        Logger {
            name: self.name.clone(),
            level_filter: Atomic::new(self.level_filter()),
            sinks: self.sinks.clone(),
            flush_level_filter: Atomic::new(self.flush_level_filter()),
            periodic_flusher: Mutex::new(None),
            error_handler: SpinRwLock::new(*self.error_handler.read()),
        }
    }
}

/// The builder of [`Logger`].
#[derive(Clone)]
pub struct LoggerBuilder {
    logger: Logger,
}

impl LoggerBuilder {
    /// Constructs a `LoggerBuilder`.
    pub fn new() -> Self {
        Self {
            logger: Logger {
                name: None,
                level_filter: Atomic::new(LevelFilter::MoreSevereEqual(Level::Info)),
                sinks: vec![],
                flush_level_filter: Atomic::new(LevelFilter::Off),
                periodic_flusher: Mutex::new(None),
                error_handler: SpinRwLock::new(None),
            },
        }
    }

    /// Sets the name of the logger.
    ///
    /// A literal constant string is usually set.
    ///
    /// # Panics
    ///
    /// A logger name should not contain any of these characters:
    /// `,` `=` `*` `?` `$` `{` `}` `"` `'` `;`,
    /// and cannot start or end with a whitespace.
    pub fn name<S>(&mut self, name: S) -> &mut Self
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.chars().any(|ch| {
            ch == ','
                || ch == '='
                || ch == '*'
                || ch == '?'
                || ch == '$'
                || ch == '{'
                || ch == '}'
                || ch == '"'
                || ch == '\''
                || ch == ';'
        }) {
            panic!("logger name contains disallowed character");
        }
        if name.starts_with(' ') || name.ends_with(' ') {
            panic!("logger name cannot start or end with a whitespace");
        }

        self.logger.name = Some(name);
        self
    }

    /// Sets the log filter level.
    #[allow(unused_mut)]
    pub fn level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.logger.set_level_filter(level_filter);
        self
    }

    /// Add a [`Sink`].
    pub fn sink(&mut self, sink: Arc<dyn Sink>) -> &mut Self {
        self.logger.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    pub fn sinks<I>(&mut self, sinks: I) -> &mut Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.logger.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Sets the flush level filter.
    #[allow(unused_mut)]
    pub fn flush_level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.logger.set_flush_level_filter(level_filter);
        self
    }

    /// Sets the error handler.
    #[allow(unused_mut)]
    pub fn error_handler(&mut self, handler: ErrorHandler) -> &mut Self {
        self.logger.set_error_handler(Some(handler));
        self
    }

    /// Builds a [`Logger`].
    pub fn build(&mut self) -> Logger {
        self.build_inner(false)
    }

    pub(crate) fn build_default(&mut self) -> Logger {
        self.build_inner(true)
    }

    fn build_inner(&mut self, is_default: bool) -> Logger {
        let res = self.logger.clone();
        let level = if is_default {
            env_level::logger_level(env_level::LoggerKind::Default)
        } else {
            env_level::logger_level(env_level::LoggerKind::Other(res.name()))
        };
        if let Some(level) = level {
            res.set_level_filter(level);
        }
        res
    }

    #[cfg(test)]
    fn build_inner_for_test(&mut self, env_level: &str, is_default: bool) -> Logger {
        let res = self.logger.clone();
        let level = if is_default {
            env_level::logger_level_inner(
                &env_level::from_str_inner(env_level).unwrap(),
                env_level::LoggerKind::Default,
            )
        } else {
            env_level::logger_level_inner(
                &env_level::from_str_inner(env_level).unwrap(),
                env_level::LoggerKind::Other(res.name()),
            )
        };
        if let Some(level) = level {
            res.set_level_filter(level);
        }
        res
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, test_utils::*};

    use std::{thread, time::Duration};

    #[test]
    fn send_sync() {
        assert_send::<Logger>();
        assert_sync::<Logger>();
    }

    #[test]
    fn flush_level() {
        let test_sink = Arc::new(CounterSink::new());
        let test_logger = Logger::builder().sink(test_sink.clone()).build();

        trace!(logger: test_logger, "");
        error!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 0);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::MoreSevereEqual(Level::Warn));
        debug!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 1);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::Off);
        info!(logger: test_logger, "");
        trace!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 0);
        test_sink.reset();

        test_logger.set_flush_level_filter(LevelFilter::MoreSevereEqual(Level::Trace));
        info!(logger: test_logger, "");
        warn!(logger: test_logger, "");
        assert_eq!(test_sink.flush_count(), 2);
        test_sink.reset();
    }

    #[test]
    fn periodic_flush() {
        let test_sink = Arc::new(CounterSink::new());
        let test_logger = Arc::new(Logger::builder().sink(test_sink.clone()).build());

        test_logger.set_flush_period(Some(Duration::from_secs(1)));

        assert_eq!(test_sink.flush_count(), 0);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 1);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 2);

        test_logger.set_flush_period(None);

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 2);

        test_logger.set_flush_period(Some(Duration::from_secs(1)));

        thread::sleep(Duration::from_millis(1250));
        assert_eq!(test_sink.flush_count(), 3);
    }

    #[test]
    fn builder_name() {
        LoggerBuilder::new().name("hello-world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_whitespace_start() {
        LoggerBuilder::new().name(" hello");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_whitespace_end() {
        LoggerBuilder::new().name("hello ");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_comma() {
        LoggerBuilder::new().name("hello,world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_eq() {
        LoggerBuilder::new().name("hello=world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_asterisk() {
        LoggerBuilder::new().name("hello*world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_question_mark() {
        LoggerBuilder::new().name("hello?world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_dollar_sign() {
        LoggerBuilder::new().name("hello$world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_curly_bracket_left() {
        LoggerBuilder::new().name("hello{world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_curly_bracket_right() {
        LoggerBuilder::new().name("hello}world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_quotation() {
        LoggerBuilder::new().name("hello\"world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_apostrophe() {
        LoggerBuilder::new().name("hello'world");
    }

    #[test]
    #[should_panic]
    fn builder_name_panic_semicolon() {
        LoggerBuilder::new().name("hello;world");
    }

    #[test]
    fn env_level() {
        macro_rules! assert_levels {
            ($env_level:literal, DEFAULT => $default:expr, UNNAMED => $unnamed:expr, NAMED($name:literal) => $named:expr $(,)?) => {
                assert_eq!(
                    Logger::builder()
                        .build_inner_for_test($env_level, true)
                        .level_filter(),
                    $default
                );
                assert_eq!(
                    Logger::builder()
                        .build_inner_for_test($env_level, false)
                        .level_filter(),
                    $unnamed
                );
                assert_eq!(
                    Logger::builder()
                        .name($name)
                        .build_inner_for_test($env_level, false)
                        .level_filter(),
                    $named
                );
            };
            (_, DEFAULT => $default:expr, UNNAMED => $unnamed:expr, NAMED($name:literal) => $named:expr $(,)?) => {
                assert_eq!(Logger::builder().build_default().level_filter(), $default);
                assert_eq!(Logger::builder().build().level_filter(), $unnamed);
                assert_eq!(Logger::builder().name($name).build().level_filter(), $named);
            };
        }

        let unchanged = LevelFilter::MoreSevereEqual(Level::Info);

        assert_levels!(
            _,
            DEFAULT => unchanged,
            UNNAMED => unchanged,
            NAMED("name") => unchanged,
        );

        assert_levels!(
            "deBug",
            DEFAULT => LevelFilter::MoreSevereEqual(Level::Debug),
            UNNAMED => unchanged,
            NAMED("name") => unchanged,
        );

        assert_levels!(
            "deBug,*=tRace",
            DEFAULT => LevelFilter::MoreSevereEqual(Level::Debug),
            UNNAMED => LevelFilter::MoreSevereEqual(Level::Trace),
            NAMED("name") => LevelFilter::MoreSevereEqual(Level::Trace),
        );

        assert_levels!(
            "=trAce",
            DEFAULT => unchanged,
            UNNAMED => LevelFilter::MoreSevereEqual(Level::Trace),
            NAMED("name") => unchanged,
        );

        assert_levels!(
            "*=waRn",
            DEFAULT => unchanged,
            UNNAMED => LevelFilter::MoreSevereEqual(Level::Warn),
            NAMED("name") => LevelFilter::MoreSevereEqual(Level::Warn),
        );

        assert_levels!(
            "=eRror,*=waRn",
            DEFAULT => unchanged,
            UNNAMED => LevelFilter::MoreSevereEqual(Level::Error),
            NAMED("name") => LevelFilter::MoreSevereEqual(Level::Warn),
        );

        assert_levels!(
            "=eRror,*=waRn,name=trAce",
            DEFAULT => unchanged,
            UNNAMED => LevelFilter::MoreSevereEqual(Level::Error),
            NAMED("name") => LevelFilter::MoreSevereEqual(Level::Trace),
        );

        assert_levels!(
            "all,*=all",
            DEFAULT => LevelFilter::All,
            UNNAMED => LevelFilter::All,
            NAMED("name") => LevelFilter::All,
        );

        assert_levels!(
            "off,*=all",
            DEFAULT => LevelFilter::Off,
            UNNAMED => LevelFilter::All,
            NAMED("name") => LevelFilter::All,
        );
    }
}
