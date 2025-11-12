use std::{fmt::Debug, result::Result as StdResult, time::Duration};

use crate::{
    env_level,
    error::{Error, ErrorHandler, InvalidArgumentError, SetLoggerNameError},
    periodic_worker::PeriodicWorker,
    sink::{Sink, Sinks},
    sync::*,
    Level, LevelFilter, Record, Result,
};

fn check_logger_name(name: impl AsRef<str>) -> StdResult<(), SetLoggerNameError> {
    let name = name.as_ref();

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
    }) || name.starts_with(' ')
        || name.ends_with(' ')
    {
        Err(SetLoggerNameError::new(name))
    } else {
        Ok(())
    }
}

/// Manages, controls and manipulates multiple sinks.
///
/// Logger is an entry point for a log to be processed, the [`Logger::log`]
/// method will be called by logging macros.
///
/// ---
///
/// A Logger:
///
/// - contains one or more [sink]s, each log will be passed into sinks in
///   sequence;
///
/// - has a level filter of its own, which is not shared with the level filter
///   of sinks;
///
/// - has automatic flushing policies that can be optionally enabled:
///   - Flush level filter: Flush once when the filter returns `true` for a log.
///   - Flush period: Flush periodically.
///
///   These two automatic flushing policies can work at the same time.
///
/// ---
///
/// *spdlog-rs* has a global [`default_logger`], you can modify it, or configure
/// a new logger to suit your needs and then replace it. Basically for a
/// lightweight program, such a global singleton logger is enough. For a complex
/// program, for example if you have a number of different components, you can
/// also configure multiple loggers and store them independently inside the
/// component struct, which allows different components to have different
/// approaches for logging.
///
/// # Examples
///
/// - Logging to the global default logger.
///
/// ```
/// use spdlog::prelude::*;
///
/// info!("logging to the default logger");
///
/// spdlog::default_logger().set_level_filter(LevelFilter::All);
/// trace!("logging to the default logger at trace level");
/// ```
///
/// - Logging to a separated logger.
///
/// ```
/// # use spdlog::prelude::*;
/// let separated_logger = /* ... */
/// # spdlog::default_logger();
///
/// info!(logger: separated_logger, "logging to a separated logger");
/// error!(logger: separated_logger, "logging to a separated logger at error level");
/// ```
///
/// - Fork, configure and replace the default logger.
///
/// ```
/// # use spdlog::prelude::*;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let new_logger = spdlog::default_logger().fork_with(|new_logger| {
///     // Configure the new logger...
///     Ok(())
/// })?;
/// spdlog::set_default_logger(new_logger);
/// info!("logging to the default logger, but it's reconfigured");
/// # Ok(()) }
/// ```
///
/// - For more examples, see [./examples] directory.
///
/// [sink]: crate::sink
/// [`default_logger`]: crate::default_logger
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
pub struct Logger {
    name: Option<String>,
    level_filter: Atomic<LevelFilter>,
    sinks: Sinks,
    flush_level_filter: Atomic<LevelFilter>,
    error_handler: RwLock<ErrorHandler>,
    periodic_flusher: Mutex<Option<(Duration, PeriodicWorker)>>,
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logger")
            .field("name", &self.name)
            .field("level_filter", &self.level_filter())
            .field(
                "sinks",
                &self
                    .sinks
                    .iter()
                    .map(|sink| sink.level_filter())
                    .collect::<Vec<_>>(),
            )
            .field("flush_level_filter", &self.flush_level_filter())
            .field("error_handler", &self.error_handler.read())
            .field(
                "periodic_flusher",
                &self
                    .periodic_flusher
                    .lock()
                    .as_deref()
                    .map(|opt| opt.as_ref().map(|(dur, _)| *dur))
                    .as_ref()
                    .map(|dur| dur as &dyn Debug)
                    .unwrap_or(&"*lock is poisoned*"),
            )
            .finish()
    }
}

impl Logger {
    /// Gets a [`LoggerBuilder`] with default parameters:
    ///
    /// | Parameter            | Default Value               |
    /// |----------------------|-----------------------------|
    /// | [name]               | `None`                      |
    /// | [sinks]              | `[]`                        |
    /// | [level_filter]       | `MoreSevereEqual(Info)`     |
    /// | [flush_level_filter] | `Off`                       |
    /// | [flush_period]       | `None`                      |
    /// | [error_handler]      | [`ErrorHandler::default()`] |
    ///
    /// [name]: LoggerBuilder::name
    /// [sinks]: LoggerBuilder::sink
    /// [level_filter]: LoggerBuilder::level_filter
    /// [flush_level_filter]: LoggerBuilder::flush_level_filter
    /// [flush_period]: Logger::set_flush_period
    /// [error_handler]: LoggerBuilder::error_handler
    /// [`ErrorHandler::default()`]: crate::error::ErrorHandler::default()
    #[must_use]
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder {
            name: None,
            level_filter: LevelFilter::MoreSevereEqual(Level::Info),
            sinks: vec![],
            flush_level_filter: LevelFilter::Off,
            error_handler: ErrorHandler::default(),
        }
    }

    /// Gets the logger name.
    ///
    /// Returns `None` if the logger does not have a name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    /// Sets the logger name.
    pub fn set_name<S>(&mut self, name: Option<S>) -> StdResult<(), SetLoggerNameError>
    where
        S: Into<String>,
    {
        if let Some(name) = name {
            let name = name.into();
            check_logger_name(&name)?;
            self.name = Some(name);
        } else {
            self.name = None;
        }
        Ok(())
    }

    /// Determines if a log with the specified level would be logged.
    ///
    /// This allows callers to avoid expensive computation of log arguments if
    /// the would be discarded anyway.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// use spdlog::prelude::*;
    /// # let logger = spdlog::default_logger();
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
    #[must_use]
    pub fn should_log(&self, level: Level) -> bool {
        self.level_filter().test(level)
    }

    /// Passes a log into sinks in sequence.
    ///
    /// It calls [`Sink::log`] method internally for each sink in sequence.
    ///
    /// # Note
    ///
    /// Users usually do not use this function directly, use logging macros
    /// instead.
    pub fn log(&self, record: &Record) {
        if !self.should_log(record.level()) {
            return;
        }
        self.sink_record(record);
    }

    /// Flushes sinks explicitly.
    ///
    /// It calls [`Sink::flush`] method internally for each sink in sequence.
    ///
    /// Users can call this function to flush explicitly and/or use automatic
    /// flushing policies. See also [`Logger::flush_level_filter`] and
    /// [`Logger::set_flush_period`].
    ///
    /// Be aware that the method can be expensive, calling it frequently may
    /// affect performance.
    pub fn flush(&self) {
        self.flush_sinks();
    }

    /// Gets the flush level filter.
    #[must_use]
    pub fn flush_level_filter(&self) -> LevelFilter {
        self.flush_level_filter.load(Ordering::Relaxed)
    }

    /// Sets a flush level filter.
    ///
    /// When logging a new record, flush the buffer if this filter condition
    /// returns `true`.
    ///
    /// This automatic flushing policy can work with
    /// [`Logger::set_flush_period`] at the same time.
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

    /// Gets the log level filter.
    #[must_use]
    pub fn level_filter(&self) -> LevelFilter {
        self.level_filter.load(Ordering::Relaxed)
    }

    /// Sets the log level filter.
    ///
    /// # Examples
    ///
    /// See [`Logger::should_log`].
    pub fn set_level_filter(&self, level_filter: LevelFilter) {
        self.level_filter.store(level_filter, Ordering::Relaxed);
    }

    /// Sets automatic periodic flushing.
    ///
    /// This function receives a `&Arc<Self>`. Calling it will spawn a new
    /// thread internally.
    ///
    /// This automatic flushing policy can work with
    /// [`Logger::set_flush_level_filter`] at the same time.
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
    /// // From now on, the `logger` will automatically call `flush` method the every 10 seconds.
    /// logger.set_flush_period(Some(Duration::from_secs(10)));
    ///
    /// // Disable automatic periodic flushing.
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
            *periodic_flusher = Some((interval, PeriodicWorker::new(callback, interval)));
        }
    }

    /// Gets a reference to sinks in the logger.
    #[must_use]
    pub fn sinks(&self) -> &[Arc<dyn Sink>] {
        &self.sinks
    }

    /// Gets a mutable reference to sinks in the logger.
    #[must_use]
    pub fn sinks_mut(&mut self) -> &mut Sinks {
        &mut self.sinks
    }

    /// Sets a error handler.
    ///
    /// If an error occurs while logging or flushing, this handler will be
    /// called. If no handler is set, the error will be print to `stderr` and
    /// then ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use spdlog::prelude::*;
    ///
    /// spdlog::default_logger().set_error_handler(|err| {
    ///     panic!("An error occurred in the default logger: {}", err)
    /// });
    /// ```
    pub fn set_error_handler<F: Into<ErrorHandler>>(&self, handler: F) {
        *self.error_handler.write_expect() = handler.into();
    }

    /// Forks and configures a separate new logger.
    ///
    /// This function creates a new logger object that inherits logger
    /// properties from `Arc<Self>`. Then this function calls the given
    /// `modifier` function which configures the properties on the new
    /// logger object. The created new logger object will be a separate
    /// object from `Arc<Self>`. (No ownership sharing)
    ///
    /// # Examples
    ///
    /// ```
    #[doc = include_str!(concat!(env!("OUT_DIR"), "/test_utils/common_for_doc_test.rs"))]
    /// # use std::sync::Arc;
    /// # use spdlog::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let test_sink = Arc::new(test_utils::StringSink::new());
    /// let old: Arc<Logger> = /* ... */
    /// # Logger::builder().build_arc().unwrap();
    /// // Fork from an existing logger and add a new sink.
    /// # let new_sink = test_sink.clone();
    /// let new = old.fork_with(|new| {
    ///     new.sinks_mut().push(new_sink);
    ///     Ok(())
    /// })?;
    ///
    /// # info!(logger: new, "first line");
    /// info!(logger: new, "this record will be written to `new_sink`");
    /// # assert_eq!(test_sink.clone_string().lines().count(), 2);
    /// info!(logger: old, "this record will not be written to `new_sink`");
    /// # assert_eq!(test_sink.clone_string().lines().count(), 2);
    /// # Ok(()) }
    /// ```
    pub fn fork_with<F>(self: &Arc<Self>, modifier: F) -> Result<Arc<Self>>
    where
        F: FnOnce(&mut Logger) -> Result<()>,
    {
        let flush_period = self.periodic_flusher.lock_expect().as_ref().map(|v| v.0);

        let mut new_logger = self.clone_lossy();
        modifier(&mut new_logger)?;

        let new_logger = Arc::new(new_logger);
        if let Some(interval) = flush_period {
            new_logger.set_flush_period(Some(interval));
        }

        Ok(new_logger)
    }

    /// Forks a separates new logger with a new name.
    ///
    /// This function creates a new logger object that inherits logger
    /// properties from `Arc<Self>` and rename the new logger object to the
    /// given name. The created new logger object will be a separate object
    /// from `Arc<Self>`. (No ownership sharing)
    ///
    /// This is a shorthand wrapper for [`Logger::fork_with`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use spdlog::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let old = Logger::builder().name("dog").build_arc()?;
    /// let new = old.fork_with_name(Some("cat"))?;
    ///
    /// assert_eq!(old.name(), Some("dog"));
    /// assert_eq!(new.name(), Some("cat"));
    /// # Ok(()) }
    /// ```
    pub fn fork_with_name<S>(self: &Arc<Self>, new_name: Option<S>) -> Result<Arc<Self>>
    where
        S: Into<String>,
    {
        self.fork_with(|new| {
            new.set_name(new_name).map_err(InvalidArgumentError::from)?;
            Ok(())
        })
    }

    // This will lose the periodic flush property, if any.
    #[must_use]
    fn clone_lossy(&self) -> Self {
        Logger {
            name: self.name.clone(),
            level_filter: Atomic::new(self.level_filter()),
            sinks: self.sinks.clone(),
            flush_level_filter: Atomic::new(self.flush_level_filter()),
            periodic_flusher: Mutex::new(None),
            error_handler: RwLock::new(self.error_handler.read_expect().clone()),
        }
    }

    fn sink_record(&self, record: &Record) {
        self.sinks.iter().for_each(|sink| {
            if sink.should_log(record.level()) {
                if let Err(err) = sink.log(record) {
                    self.handle_error(err);
                }
            }
        });

        if self.should_flush(record) {
            self.flush();
        }
    }

    fn flush_sinks_with(&self, with: impl Fn(&dyn Sink) -> Result<()>) {
        self.sinks.iter().for_each(|sink| {
            if let Err(err) = with(&**sink) {
                self.handle_error(err);
            }
        });
    }

    pub(crate) fn flush_sinks_on_exit(&self) {
        self.flush_sinks_with(|sink| sink.flush_on_exit());
    }

    pub(crate) fn flush_sinks(&self) {
        self.flush_sinks_with(|sink| sink.flush());
    }

    fn handle_error(&self, err: Error) {
        self.error_handler.read_expect().call_internal(
            format!(
                "Logger ({})",
                self.name.as_ref().map_or("*no name*", String::as_str)
            ),
            err,
        );
    }

    #[must_use]
    fn should_flush(&self, record: &Record) -> bool {
        self.flush_level_filter().test(record.level())
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
        self.clone_lossy()
    }
}

#[allow(missing_docs)]
#[derive(Clone)]
pub struct LoggerBuilder {
    name: Option<String>,
    level_filter: LevelFilter,
    sinks: Sinks,
    flush_level_filter: LevelFilter,
    error_handler: ErrorHandler,
}

impl LoggerBuilder {
    /// Constructs a `LoggerBuilder`.
    #[allow(clippy::new_without_default)]
    #[deprecated(
        since = "0.3.0",
        note = "it may be removed in the future, use `Logger::builder()` instead"
    )]
    #[must_use]
    pub fn new() -> Self {
        Logger::builder()
    }

    /// Sets the name of the logger.
    ///
    /// This parameter is **optional**.
    ///
    /// # Requirements
    ///
    /// A logger name should not contain any of these characters:
    /// `,` `=` `*` `?` `$` `{` `}` `"` `'` `;`,
    /// and cannot start or end with a whitespace.
    ///
    /// Otherwise, [`LoggerBuilder::build`] will return an error.
    pub fn name<S>(&mut self, name: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Sets the log level filter.
    ///
    /// This parameter is **optional**.
    pub fn level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.level_filter = level_filter;
        self
    }

    /// Add a [`Sink`].
    pub fn sink(&mut self, sink: Arc<dyn Sink>) -> &mut Self {
        self.sinks.push(sink);
        self
    }

    /// Add multiple [`Sink`]s.
    pub fn sinks<I>(&mut self, sinks: I) -> &mut Self
    where
        I: IntoIterator<Item = Arc<dyn Sink>>,
    {
        self.sinks.append(&mut sinks.into_iter().collect());
        self
    }

    /// Sets the flush level filter.
    ///
    /// This parameter is **optional**.
    ///
    /// See the documentation of [`Logger::set_flush_level_filter`] for the
    /// description of this parameter.
    pub fn flush_level_filter(&mut self, level_filter: LevelFilter) -> &mut Self {
        self.flush_level_filter = level_filter;
        self
    }

    /// Sets the error handler.
    ///
    /// This parameter is **optional**.
    ///
    /// See the documentation of [`Logger::set_error_handler`] for the
    /// description of this parameter.
    pub fn error_handler<F: Into<ErrorHandler>>(&mut self, handler: F) -> &mut Self {
        self.error_handler = handler.into();
        self
    }

    /// Builds a [`Logger`].
    pub fn build(&mut self) -> Result<Logger> {
        self.build_inner(self.preset_level(false))
    }

    /// Builds a `Arc<Logger>`.
    ///
    /// This is a shorthand method for `.build().map(Arc::new)`.
    ///
    /// ## Notes
    ///
    /// Unlike [`Sink`], which almost always needs to work with [`Arc`].
    /// [`Logger`] can be used directly in logging macros without `Arc`. So
    /// when `Arc<Logger>` is not needed, users should just call
    /// [`LoggerBuilder::build`] to get a `Logger` without `Arc`.
    pub fn build_arc(&mut self) -> Result<Arc<Logger>> {
        self.build().map(Arc::new)
    }

    pub(crate) fn build_default(&mut self) -> Result<Logger> {
        self.build_inner(self.preset_level(true))
    }

    #[must_use]
    fn preset_level(&self, is_default: bool) -> Option<LevelFilter> {
        if is_default {
            env_level::logger_level(env_level::LoggerKind::Default)
        } else {
            env_level::logger_level(env_level::LoggerKind::Other(self.name.as_deref()))
        }
    }

    fn build_inner(&mut self, preset_level: Option<LevelFilter>) -> Result<Logger> {
        if let Some(name) = &self.name {
            check_logger_name(name).map_err(InvalidArgumentError::from)?;
        }

        let logger = Logger {
            name: self.name.clone(),
            level_filter: Atomic::new(self.level_filter),
            sinks: self.sinks.clone(),
            flush_level_filter: Atomic::new(self.flush_level_filter),
            error_handler: RwLock::new(self.error_handler.clone()),
            periodic_flusher: Mutex::new(None),
        };

        if let Some(preset_level) = preset_level {
            logger.set_level_filter(preset_level);
        }

        Ok(logger)
    }

    #[cfg(test)]
    #[must_use]
    fn build_inner_for_test(&mut self, env_level: &str, is_default: bool) -> Logger {
        let preset_level = if is_default {
            env_level::logger_level_inner(
                &env_level::from_str_inner(env_level).unwrap(),
                env_level::LoggerKind::Default,
            )
        } else {
            env_level::logger_level_inner(
                &env_level::from_str_inner(env_level).unwrap(),
                env_level::LoggerKind::Other(self.name.as_deref()),
            )
        };

        self.build_inner(preset_level).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    use crate::{prelude::*, test_utils::*};

    #[test]
    fn logger_traits() {
        assert_trait!(Logger: Send + Sync + Debug);
    }

    #[test]
    fn flush_level() {
        let test_sink = Arc::new(TestSink::new());
        let test_logger = Logger::builder().sink(test_sink.clone()).build().unwrap();

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
        let test_sink = Arc::new(TestSink::new());
        let test_logger = Logger::builder()
            .sink(test_sink.clone())
            .build_arc()
            .unwrap();

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
        Logger::builder().name("hello-world");

        macro_rules! assert_name_err {
            ( $($name:literal),+ $(,)? ) => {
                $(match Logger::builder().name($name).build() {
                    Err(Error::InvalidArgument(InvalidArgumentError::LoggerName(err))) => {
                        assert_eq!(err.name(), $name)
                    }
                    _ => panic!("test case '{}' failed", $name),
                })+
            };
        }

        assert_name_err! {
            " hello", "hello ",
            "hello,world", "hello=world", "hello*world", "hello?world", "hello$world",
            "hello{world", "hello}world", r#"hello"world"#, "hello'world", "hello;world",
        };
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
                assert_eq!(
                    Logger::builder().build_default().unwrap().level_filter(),
                    $default
                );
                assert_eq!(Logger::builder().build().unwrap().level_filter(), $unnamed);
                assert_eq!(
                    Logger::builder()
                        .name($name)
                        .build()
                        .unwrap()
                        .level_filter(),
                    $named
                );
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

    #[test]
    fn fork_logger() {
        let test_sink = (Arc::new(TestSink::new()), Arc::new(TestSink::new()));
        let logger = Arc::new(build_test_logger(|b| b.sink(test_sink.0.clone())));

        assert!(logger.name().is_none());
        assert_eq!(test_sink.0.log_count(), 0);
        assert_eq!(test_sink.0.flush_count(), 0);
        assert_eq!(test_sink.1.log_count(), 0);
        assert_eq!(test_sink.1.flush_count(), 0);

        info!(logger: logger, "qwq");
        assert!(logger.name().is_none());
        assert_eq!(test_sink.0.log_count(), 1);
        assert_eq!(test_sink.0.flush_count(), 0);
        assert_eq!(test_sink.1.log_count(), 0);
        assert_eq!(test_sink.1.flush_count(), 0);

        let old = logger;
        let new = old.fork_with_name(Some("cat")).unwrap();
        info!(logger: new, "meow");
        assert!(old.name().is_none());
        assert_eq!(new.name(), Some("cat"));
        assert_eq!(test_sink.0.log_count(), 2);
        assert_eq!(test_sink.0.flush_count(), 0);
        assert_eq!(test_sink.1.log_count(), 0);
        assert_eq!(test_sink.1.flush_count(), 0);

        let old = new;
        let new = old
            .fork_with(|new| {
                new.set_name(Some("dog")).unwrap();
                new.sinks_mut().push(test_sink.1.clone());
                Ok(())
            })
            .unwrap();
        info!(logger: new, "woof");
        assert_eq!(old.name(), Some("cat"));
        assert_eq!(new.name(), Some("dog"));
        assert_eq!(test_sink.0.log_count(), 3);
        assert_eq!(test_sink.0.flush_count(), 0);
        assert_eq!(test_sink.1.log_count(), 1);
        assert_eq!(test_sink.1.flush_count(), 0);

        assert!(matches!(
            new.fork_with_name(Some("invalid,name")),
            Err(Error::InvalidArgument(InvalidArgumentError::LoggerName(_)))
        ));

        assert!(new
            .fork_with_name(None as Option<&str>)
            .unwrap()
            .name()
            .is_none());

        let test_sink = (Arc::new(TestSink::new()), Arc::new(TestSink::new()));
        let old = Arc::new(build_test_logger(|b| b.sink(test_sink.0.clone())));
        old.set_flush_period(Some(Duration::from_secs(1)));
        std::thread::sleep(Duration::from_millis(1250));

        let _new = old
            .fork_with(|new| {
                new.sinks_mut().clear();
                new.sinks_mut().push(test_sink.1.clone());
                Ok(())
            })
            .unwrap();
        std::thread::sleep(Duration::from_millis(1250));

        assert_eq!(test_sink.0.log_count(), 0);
        assert_eq!(test_sink.0.flush_count(), 2);
        assert_eq!(test_sink.1.log_count(), 0);
        assert_eq!(test_sink.1.flush_count(), 1);
    }
}
