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
    ({ $($ttm:tt)+ }) => {
        $crate::__kv!(@{} $($ttm)+)
    };

    (@{$($done:tt)*} $k:ident    = $v:expr $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [  ] = $v,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :  = $v:expr $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [: ] = $v,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :? = $v:expr $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [:?] = $v,} $($($rest)*)?));
    (@{$( $k:ident [$($modifier:tt)*] = $v:expr ),+ $(,)?}) => {
        &[$(($crate::kv::Key::__from_static_str(stringify!($k)), $crate::__kv_value!($($modifier)* = $v))),+]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kv_value {
    (= $v:expr) => {
        $crate::kv::Value::from(&$v)
    };
    (: = $v:expr) => {
        $crate::kv::Value::from_display(&$v)
    };
    (:? = $v:expr) => {
        $crate::kv::Value::from_debug(&$v)
    };
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self, Debug, Display},
        sync::Arc,
    };

    use crate::{kv::KeyInner, prelude::*, test_utils::*};

    #[test]
    fn syntax_and_records() {
        let test_sink = Arc::new(TestSink::new());
        let test = Arc::new(build_test_logger(|b| {
            b.sink(test_sink.clone()).level_filter(LevelFilter::All)
        }));

        struct Mods;
        impl Debug for Mods {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "debug")
            }
        }
        impl Display for Mods {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "display")
            }
        }

        let mut check = vec![
            (vec![], Level::Info, "logger".to_string()),
            (vec![], Level::Error, "logger, kv(0)".to_string()),
            (
                vec![(KeyInner::StaticStr("kn"), "114514".to_string())],
                Level::Warn,
                "logger, kv(1)".to_string(),
            ),
            (
                vec![
                    (KeyInner::StaticStr("kn"), "114514".to_string()),
                    (KeyInner::StaticStr("kdi"), "display".to_string()),
                    (KeyInner::StaticStr("kde"), "debug".to_string()),
                ],
                Level::Critical,
                "logger, kv(2)".to_string(),
            ),
        ];

        log!(logger: test, Level::Info, "logger");
        log!(logger: test, kv: {}, Level::Error, "logger, kv(0)");
        log!(logger: test, kv: { kn = 114514 }, Level::Warn, "logger, kv(1)");
        log!(logger: test, kv: { kn = 114514, kdi: = Mods, kde:? = Mods }, Level::Critical, "logger, kv(2)");

        macro_rules! add_records {
            ( $($level:ident => $variant:ident),+ ) => {
                $(
                    $level!(logger: test, "{}: logger", stringify!($level));
                    check.push((vec![], Level::$variant, format!("{}: logger", stringify!($level))));

                    $level!(logger: test, kv: {}, "{}: logger, kv(0)", stringify!($level));
                    check.push((vec![], Level::$variant, format!("{}: logger, kv(0)", stringify!($level))));

                    $level!(logger: test, kv: { kn = 114514 }, "{}: logger, kv(1)", stringify!($level));
                    check.push((
                        vec![(KeyInner::StaticStr("kn"), "114514".to_string())],
                        Level::$variant,
                        format!("{}: logger, kv(1)", stringify!($level))
                    ));

                    $level!(logger: test, kv: { kn = 114514, kdi: = Mods, kde:? = Mods }, "{}: logger, kv(s,mod)", stringify!($level));
                    check.push((
                        vec![
                            (KeyInner::StaticStr("kn"), "114514".to_string()),
                            (KeyInner::StaticStr("kdi"), "display".to_string()),
                            (KeyInner::StaticStr("kde"), "debug".to_string()),
                        ],
                        Level::$variant,
                        format!("{}: logger, kv(s,mod)", stringify!($level))
                    ));
                )+
            };
        }
        add_records!(
            critical => Critical,
            error => Error,
            warn => Warn,
            info => Info,
            debug => Debug,
            trace => Trace
        );

        let records = test_sink.records();
        let from_sink = records
            .iter()
            .map(|record| {
                (
                    record
                        .key_values()
                        .map(|(k, v)| (k.inner(), v.to_string()))
                        .collect::<Vec<_>>(),
                    record.level(),
                    record.payload().to_string(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(check, from_sink);
    }
}
