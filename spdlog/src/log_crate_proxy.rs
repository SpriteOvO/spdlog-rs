use std::time::SystemTime;

use crate::{default_logger, sync::*, utils, LogCrateRecord, Logger};

/// Proxy layer for compatible [log crate].
///
/// Call [`init_log_crate_proxy`] to initialize the proxy, and then configure
/// the proxy via [`log_crate_proxy`].
///
/// After the proxy is initialized, it will forward all log messages from `log`
/// crate to the global default logger or the logger set by
/// [`LogCrateProxy::set_logger`].
///
/// To set filters or read from `RUST_LOG` variable, call
/// [`LogCrateProxy::set_filter`] after initialization.
///
/// Note that the `log` crate uses a different log level filter and by default
/// it rejects all log messages. To make `LogCrateProxy` able to receive log
/// messages from `log` crate, you may need to call
/// [`re_export::log::set_max_level`] with [`re_export::log::LevelFilter`].
///
/// ## Examples
///
/// ```
/// use spdlog::re_export::log;
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
/// [log crate]: https://crates.io/crates/log
/// [`init_log_crate_proxy`]: crate::init_log_crate_proxy
/// [`log_crate_proxy`]: crate::log_crate_proxy()
/// [`re_export::log::set_max_level`]: crate::re_export::log::set_max_level
/// [`re_export::log::LevelFilter`]: crate::re_export::log::LevelFilter
/// [./examples]: https://github.com/SpriteOvO/spdlog-rs/tree/main/spdlog/examples
#[derive(Default)]
pub struct LogCrateProxy {
    logger: ArcSwapOption<Logger>,
    filter: ArcSwapOption<env_filter::Filter>,
}

impl LogCrateProxy {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Sets a logger as the new receiver, and returens the old one.
    ///
    /// If the argument `logger` is `None`, the global default logger will be
    /// used.
    pub fn swap_logger(&self, logger: Option<Arc<Logger>>) -> Option<Arc<Logger>> {
        self.logger.swap(logger)
    }

    /// Sets a logger as the new receiver.
    ///
    /// If the argument `logger` is `None`, the global default logger will be
    /// used.
    pub fn set_logger(&self, logger: Option<Arc<Logger>>) {
        self.swap_logger(logger);
    }

    /// Sets a filter for records from `log` crate.
    ///
    /// This is useful if users want to support `RUST_LOG` environment variable.
    pub fn set_filter(&self, filter: Option<env_filter::Filter>) {
        self.filter.swap(filter.map(Arc::new));
    }

    #[must_use]
    fn logger(&self) -> Arc<Logger> {
        self.logger.load_full().unwrap_or_else(default_logger)
    }
}

impl log::Log for LogCrateProxy {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let filter = self.filter.load();
        utils::is_none_or(filter.as_deref(), |filter| filter.enabled(metadata))
            && self.logger().should_log(metadata.level().into())
    }

    fn log(&self, record: &log::Record) {
        if utils::is_none_or(self.filter.load().as_deref(), |filter| {
            filter.matches(record)
        }) {
            let logger = self.logger();
            let record = LogCrateRecord::new(&logger, record, SystemTime::now());
            logger.log(&record.as_record())
        }
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

        let sink = Arc::new(TestSink::new());
        crate::log_crate_proxy()
            .set_logger(Some(Arc::new(build_test_logger(|b| b.sink(sink.clone())))));
        crate::log_crate_proxy().set_filter(Some(
            env_filter::Builder::new()
                .filter_module(
                    "spdlog::log_crate_proxy::tests::should_be_filtered_out",
                    log::LevelFilter::Off,
                )
                .filter(None, log::LevelFilter::Trace)
                .build(),
        ));

        assert_eq!(sink.log_count(), 0);

        log::info!("hello");
        log::error!("world");
        mod should_be_filtered_out {
            pub fn log_something() {
                log::warn!("this should be filtered out");
            }
        }
        should_be_filtered_out::log_something();

        assert_eq!(sink.log_count(), 2);
        assert_eq!(
            sink.payloads(),
            vec!["hello".to_string(), "world".to_string()]
        );
    }
}
