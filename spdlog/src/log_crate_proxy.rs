use std::time::SystemTime;

/// Re-export some stuff from `log` crate for convenience.
///
/// Users sometimes need these stuff, re-exporting them eliminates the need to
/// explicitly depend on `log` crate in `Cargo.toml`.
///
/// See the documentation of [`LogCrateProxy`].
#[cfg(feature = "log")] // Intentionally redundant, workaround for defects in nested exports of feature
                        // `doc_auto_cfg`.
pub mod log_crate {
    pub use log::{set_max_level, LevelFilter, SetLoggerError};
}

use crate::{default_logger, sync::*, Logger, Record};

/// Log crate proxy.
///
/// It forwards all log messages from `log` crate to [`default_logger`] by
/// default, and you can set a separate logger for it via
/// [`LogCrateProxy::set_logger`].
///
/// If upstream dependencies use `log` crate to output log messages, they may
/// also be received by `LogCrateProxy`.
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To make `LogCrateProxy` able to receive log
/// messages from `log` crate, you may need to call [`log_crate::set_max_level`]
/// with [`log_crate::LevelFilter`].
///
/// ## Examples
///
/// ```
/// use spdlog::log_crate as log;
///
/// # fn main() -> Result<(), log::SetLoggerError> {
/// spdlog::init_log_crate_proxy()?;
/// // Enable all log messages from `log` crate.
/// log::set_max_level(log::LevelFilter::Trace);
/// # Ok(()) }
/// ```
///
/// For more and detailed examples, see [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
#[derive(Default)]
pub struct LogCrateProxy {
    logger: ArcSwapOption<Logger>,
}

impl LogCrateProxy {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Swaps a logger.
    ///
    /// If the argument `logger` is `None`, the return value of
    /// [`default_logger`] will be used.
    pub fn swap_logger(&self, logger: Option<Arc<Logger>>) -> Option<Arc<Logger>> {
        self.logger.swap(logger)
    }

    /// Sets a logger.
    ///
    /// If the argument `logger` is `None`, the return value of
    /// [`default_logger`] will be used.
    pub fn set_logger(&self, logger: Option<Arc<Logger>>) {
        self.swap_logger(logger);
    }

    fn logger(&self) -> Arc<Logger> {
        self.logger.load_full().unwrap_or_else(default_logger)
    }
}

impl log::Log for LogCrateProxy {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger().should_log(metadata.level().into())
    }

    fn log(&self, record: &log::Record) {
        let logger = self.logger();
        let record = Record::from_log_crate_record(&logger, record, SystemTime::now());
        logger.log(&record)
    }

    fn flush(&self) {
        self.logger().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn proxy() {
        crate::init_log_crate_proxy().unwrap();
        log::set_max_level(log::LevelFilter::Debug);

        let sink = Arc::new(CounterSink::new());
        crate::log_crate_proxy().set_logger(Some(Arc::new(
            test_logger_builder().sink(sink.clone()).build().unwrap(),
        )));

        assert_eq!(sink.log_count(), 0);

        log::info!("hello");
        log::error!("world");

        assert_eq!(sink.log_count(), 2);
        assert_eq!(
            sink.payloads(),
            vec!["hello".to_string(), "world".to_string()]
        );
    }
}
