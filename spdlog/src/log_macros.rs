/// Logs a message at the specified level.
///
/// This macro will generically log with the specified [`Level`] and `format!`
/// based argument list.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::{log, Level};
///
/// # let app_events = spdlog::default_logger();
/// let data = (42, "Forty-two");
///
/// // Using the global default logger
/// log!(Level::Info, "Received data: {}, {}", data.0, data.1);
///
/// // Or using the specified logger
/// log!(logger: app_events, Level::Info, "Received data: {}, {}", data.0, data.1);
/// ```
///
/// [`Level`]: crate::Level
#[macro_export]
macro_rules! log {
    (logger: $logger:expr, kv: $kv:tt, $level:expr, $($arg:tt)+) => ({
        let logger = &$logger;
        const LEVEL: $crate::Level = $level;
        const SHOULD_LOG: bool = $crate::STATIC_LEVEL_FILTER.__test_const(LEVEL);
        if SHOULD_LOG && logger.should_log(LEVEL) {
            $crate::__log(logger, LEVEL, $crate::source_location_current!(), $crate::__kv!($kv), format_args!($($arg)+));
        }
    });
    (logger: $logger:expr, $level:expr, $($arg:tt)+) => ($crate::log!(logger: $logger, kv: {}, $level, $($arg)+));
    (kv: $kv:tt, $level:expr, $($arg:tt)+) => ($crate::log!(logger: $crate::default_logger(), kv: $kv, $level, $($arg)+));
    ($level:expr, $($arg:tt)+) => ($crate::log!(logger: $crate::default_logger(), kv: {}, $level, $($arg)+));
}

/// Logs a message at the critical level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::critical;
///
/// # let app_events = spdlog::default_logger();
/// let (left, right) = (true, false);
///
/// // Using the global default logger
/// critical!("Runtime assertion failed. Left: `{}`, Right: `{}`", left, right);
///
/// // Or using the specified logger
/// critical!(logger: app_events, "Runtime assertion failed. Left: `{}`, Right: `{}`", left, right);
/// ```
#[macro_export]
macro_rules! critical {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Critical, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Critical, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Critical, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Critical, $($arg)+)
    )
}

/// Logs a message at the error level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::error;
///
/// # let app_events = spdlog::default_logger();
/// let (err_info, port) = ("No connection", 22);
///
/// // Using the global default logger
/// error!("Error: {} on port {}", err_info, port);
///
/// // Or using the specified logger
/// error!(logger: app_events, "App Error: {}, Port: {}", err_info, port);
/// ```
#[macro_export]
macro_rules! error {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Error, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Error, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Error, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Error, $($arg)+)
    )
}

/// Logs a message at the warn level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::warn;
///
/// # let input_events = spdlog::default_logger();
/// let warn_description = "Invalid Input";
///
/// // Using the global default logger
/// warn!("Warning! {}!", warn_description);
///
/// // Or using the specified logger
/// warn!(logger: input_events, "App received warning: {}", warn_description);
/// ```
#[macro_export]
macro_rules! warn {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Warn, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Warn, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Warn, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Warn, $($arg)+)
    )
}

/// Logs a message at the info level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::info;
///
/// # struct Connection { port: u32, speed: f32 }
/// # let conn_events = spdlog::default_logger();
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// // Using the global default logger
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
///
/// // Or using the specified logger
/// info!(logger: conn_events, "Successfull connection, port: {}, speed: {}", conn_info.port, conn_info.speed);
/// ```
#[macro_export]
macro_rules! info {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Info, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Info, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Info, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Info, $($arg)+)
    )
}

/// Logs a message at the debug level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::debug;
///
/// # struct Position { x: f32, y: f32 }
/// # let app_events = spdlog::default_logger();
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// // Using the global default logger
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
///
/// // Or using the specified logger
/// debug!(logger: app_events, "New position: x: {}, y: {}", pos.x, pos.y);
/// ```
#[macro_export]
macro_rules! debug {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Debug, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Debug, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Debug, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Debug, $($arg)+)
    )
}

/// Logs a message at the trace level.
#[doc = include_str!("./include/doc/log-macro-nameed-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::trace;
///
/// # struct Position { x: f32, y: f32 }
/// # let app_events = spdlog::default_logger();
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// // Using the global default logger
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
///
/// // Or using the specified logger
/// trace!(logger: app_events, "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// ```
#[macro_export]
macro_rules! trace {
    (logger: $logger:expr, kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(logger: $logger, kv: $kv, $crate::Level::Trace, $($arg)+)
    );
    (logger: $logger:expr, $($arg:tt)+) => (
        $crate::log!(logger: $logger, $crate::Level::Trace, $($arg)+)
    );
    (kv: $kv:tt, $($arg:tt)+) => (
        $crate::log!(kv: $kv, $crate::Level::Trace, $($arg)+)
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::Level::Trace, $($arg)+)
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kv {
    ({}) => (&[]);
    ({ $( $k:ident = $v:expr ),+ $(,)? }) => {
        &[$(($crate::kv::Key::__from_static_str(stringify!($k)), $v.into())),+]
    };
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{kv::KeyInner, prelude::*, test_utils::*};

    #[test]
    fn syntax_and_records() {
        let test_sink = Arc::new(TestSink::new());
        let test = Arc::new(build_test_logger(|b| {
            b.sink(test_sink.clone()).level_filter(LevelFilter::All)
        }));

        log!(logger: test, Level::Info, "logger");
        log!(logger: test, kv: {}, Level::Error, "logger, kv(0)");
        log!(logger: test, kv: { k1 = 114 }, Level::Warn, "logger, kv(1)");
        log!(logger: test, kv: { k1 = 114, k2 = 514 }, Level::Critical, "logger, kv(2)");
        critical!(logger: test, "critical: logger");
        critical!(logger: test, kv: {}, "critical: logger, kv(0)");
        critical!(logger: test, kv: { k1 = 114 }, "critical: logger, kv(1)");
        critical!(logger: test, kv: { k1 = 114, k2 = 514 }, "critical: logger, kv(2)");
        error!(logger: test, "error: logger");
        error!(logger: test, kv: {}, "error: logger, kv(0)");
        error!(logger: test, kv: { k1 = 114 }, "error: logger, kv(1)");
        error!(logger: test, kv: { k1 = 114, k2 = 514 }, "error: logger, kv(2)");
        warn!(logger: test, "warn: logger");
        warn!(logger: test, kv: {}, "warn: logger, kv(0)");
        warn!(logger: test, kv: { k1 = 114 }, "warn: logger, kv(1)");
        warn!(logger: test, kv: { k1 = 114, k2 = 514 }, "warn: logger, kv(2)");
        info!(logger: test, "info: logger");
        info!(logger: test, kv: {}, "info: logger, kv(0)");
        info!(logger: test, kv: { k1 = 114 }, "info: logger, kv(1)");
        info!(logger: test, kv: { k1 = 114, k2 = 514 }, "info: logger, kv(2)");
        debug!(logger: test, "debug: logger");
        debug!(logger: test, kv: {}, "debug: logger, kv(0)");
        debug!(logger: test, kv: { k1 = 114 }, "debug: logger, kv(1)");
        debug!(logger: test, kv: { k1 = 114, k2 = 514 }, "debug: logger, kv(2)");
        trace!(logger: test, "trace: logger");
        trace!(logger: test, kv: {}, "trace: logger, kv(0)");
        trace!(logger: test, kv: { k1 = 114 }, "trace: logger, kv(1)");
        trace!(logger: test, kv: { k1 = 114, k2 = 514 }, "trace: logger, kv(2)");

        let records = test_sink.records();
        let check = records
            .iter()
            .map(|record| {
                (
                    record
                        .key_values()
                        .map(|(k, v)| (k.inner(), v.to_i64().unwrap()))
                        .collect::<Vec<_>>(),
                    record.level(),
                    record.payload(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            check,
            vec![
                (vec![], Level::Info, "logger"),
                (vec![], Level::Error, "logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Warn,
                    "logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Critical,
                    "logger, kv(2)"
                ),
                //
                (vec![], Level::Critical, "critical: logger"),
                (vec![], Level::Critical, "critical: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Critical,
                    "critical: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Critical,
                    "critical: logger, kv(2)"
                ),
                //
                (vec![], Level::Error, "error: logger"),
                (vec![], Level::Error, "error: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Error,
                    "error: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Error,
                    "error: logger, kv(2)"
                ),
                //
                (vec![], Level::Warn, "warn: logger"),
                (vec![], Level::Warn, "warn: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Warn,
                    "warn: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Warn,
                    "warn: logger, kv(2)"
                ),
                //
                (vec![], Level::Info, "info: logger"),
                (vec![], Level::Info, "info: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Info,
                    "info: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Info,
                    "info: logger, kv(2)"
                ),
                //
                (vec![], Level::Debug, "debug: logger"),
                (vec![], Level::Debug, "debug: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Debug,
                    "debug: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Debug,
                    "debug: logger, kv(2)"
                ),
                //
                (vec![], Level::Trace, "trace: logger"),
                (vec![], Level::Trace, "trace: logger, kv(0)"),
                (
                    vec![(KeyInner::StaticStr("k1"), 114)],
                    Level::Trace,
                    "trace: logger, kv(1)"
                ),
                (
                    vec![
                        (KeyInner::StaticStr("k1"), 114),
                        (KeyInner::StaticStr("k2"), 514)
                    ],
                    Level::Trace,
                    "trace: logger, kv(2)"
                ),
            ]
        );
    }
}
