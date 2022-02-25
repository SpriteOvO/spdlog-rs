use std::time::SystemTime;

use crate::{default_logger, sync::*, Logger, Record};

/// Log crate proxy.
///
/// It forwards all logs from log crate to [`default_logger`] by default, and
/// you can set a separate logger for it via [`LogCrateProxy::set_logger`].
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To log messages via the `log` crate, you have
/// to call [`log::set_max_level`] manually before logging. For more
/// information, please read the documentation of [`log::set_max_level`].
///
/// ## Examples
///
/// See [./examples] directory.
///
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/examples
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
            test_logger_builder().sink(sink.clone()).build(),
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
