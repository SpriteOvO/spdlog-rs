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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}], $($input)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_impl {
    (logger: $logger:expr, kv: $kv:tt, $level:expr, $($arg:tt)+) => ({
        let logger = &$logger;
        const LEVEL: $crate::Level = $level;
        const SHOULD_LOG: bool = $crate::STATIC_LEVEL_FILTER.__test_const(LEVEL);
        if SHOULD_LOG && logger.should_log(LEVEL) {
            $crate::__log(logger, LEVEL, $crate::source_location_current!(), $crate::__kv!($kv), format_args!($($arg)+));
        }
    });
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Critical], $($input)+)
    };
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Error], $($input)+)
    };
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Warn], $($input)+)
    };
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Info], $($input)+)
    };
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Debug], $($input)+)
    };
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
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, $crate::Level::Trace], $($input)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kv {
    ({}) => (&[]);
    ({ $($ttm:tt)+ }) => {
        $crate::__kv!(@{} $($ttm)+)
    };

    (@{$($done:tt)*} $k:ident    $(= $v:expr)? $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [  ] $(= $v)?,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :  $(= $v:expr)? $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [: ] $(= $v)?,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :? $(= $v:expr)? $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [:?] $(= $v)?,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :sval $(= $v:expr)? $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [:sval] $(= $v)?,} $($($rest)*)?));
    (@{$($done:tt)*} $k:ident :serde $(= $v:expr)? $(,$($rest:tt)*)?) => ($crate::__kv!(@{$($done)* $k [:serde] $(= $v)?,} $($($rest)*)?));
    (@{$( $k:ident [$($modifier:tt)*] $(= $v:expr)? ),+ $(,)?}) => {
        &[$(($crate::kv::Key::__from_static_str(stringify!($k)), $crate::__kv_value!($k [$($modifier)*] $(= $v)?))),+]
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kv_value {
    ($k:ident [$($modifier:tt)*]) => { $crate::__kv_value!($k [$($modifier)*] = $k) };
    ($k:ident [  ] = $v:expr) => { $crate::kv::Value::from(&$v) };
    ($k:ident [: ] = $v:expr) => { $crate::kv::Value::capture_display(&$v) };
    ($k:ident [:?] = $v:expr) => { $crate::kv::Value::capture_debug(&$v) };
    ($k:ident [:sval] = $v:expr) => { $crate::kv::Value::capture_sval2(&$v) };
    ($k:ident [:serde] = $v:expr) => { $crate::kv::Value::capture_serde1(&$v) };
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self, Debug, Display},
        sync::Arc,
        vec,
    };

    use crate::{
        formatter::Formatter,
        kv::{Key, KeyInner},
        prelude::*,
        sink::Sink,
        test_utils::{self, *},
        ErrorHandler, Record,
    };

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
            (
                vec![(KeyInner::StaticStr("n"), "114514".to_string())],
                Level::Trace,
                "logger, kv(1,vref)".to_string(),
            ),
            (
                vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                Level::Debug,
                "logger, kv(mod,vref)".to_string(),
            ),
            (
                vec![
                    (KeyInner::StaticStr("n"), "114514".to_string()),
                    (KeyInner::StaticStr("mod_di"), "display".to_string()),
                    (KeyInner::StaticStr("mod_de"), "debug".to_string()),
                ],
                Level::Info,
                "logger, kv(s,mod,vref)".to_string(),
            ),
            (
                vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                Level::Debug,
                "arbitrary order = logger, fmt, kv".to_string(),
            ),
            (
                vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                Level::Debug,
                "arbitrary order = fmt, logger, kv".to_string(),
            ),
        ];

        log!(logger: test, Level::Info, "logger");
        log!(logger: test, kv: {}, Level::Error, "logger, kv(0)");
        log!(logger: test, kv: { kn = 114514 }, Level::Warn, "logger, kv(1)");
        log!(logger: test, kv: { kn = 114514, kdi: = Mods, kde:? = Mods }, Level::Critical, "logger, kv(2)");

        let (n, mod_di, mod_de) = (114514, Mods, Mods);
        log!(logger: test, kv: { n }, Level::Trace, "logger, kv(1,vref)");
        log!(logger: test, kv: { mod_di: }, Level::Debug, "logger, kv(mod,vref)");
        log!(logger: test, kv: { n, mod_di:, mod_de:? }, Level::Info, "logger, kv(s,mod,vref)");
        log!(logger: test, Level::Debug, "arbitrary order = logger, fmt, kv", kv: { mod_di: });
        log!(Level::Debug, "arbitrary order = fmt, logger, kv", logger: test, kv: { mod_di: });

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

                    $level!(logger: test, kv: { n }, "{}: logger, kv(1,vref)", stringify!($level));
                    check.push((
                        vec![(KeyInner::StaticStr("n"), "114514".to_string())],
                        Level::$variant,
                        format!("{}: logger, kv(1,vref)", stringify!($level))
                    ));

                    $level!(logger: test, kv: { mod_di: }, "{}: logger, kv(mod,vref)", stringify!($level));
                    check.push((
                        vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                        Level::$variant,
                        format!("{}: logger, kv(mod,vref)", stringify!($level))
                    ));

                    $level!(logger: test, kv: { n, mod_di:, mod_de:? }, "{}: logger, kv(s,mod,vref)", stringify!($level));
                    check.push((
                        vec![
                            (KeyInner::StaticStr("n"), "114514".to_string()),
                            (KeyInner::StaticStr("mod_di"), "display".to_string()),
                            (KeyInner::StaticStr("mod_de"), "debug".to_string()),
                        ],
                        Level::$variant,
                        format!("{}: logger, kv(s,mod,vref)", stringify!($level))
                    ));

                    $level!(logger: test, "{}: arbitrary order = logger, fmt, kv", stringify!($level), kv: { mod_di: });
                    check.push((
                        vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                        Level::$variant,
                        format!("{}: arbitrary order = logger, fmt, kv", stringify!($level))
                    ));

                    $level!("{}: arbitrary order = fmt, logger, kv", stringify!($level), logger: test, kv: { mod_di: });
                    check.push((
                        vec![(KeyInner::StaticStr("mod_di"), "display".to_string())],
                        Level::$variant,
                        format!("{}: arbitrary order = fmt, logger, kv", stringify!($level))
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
                        .into_iter()
                        .map(|(k, v)| (k.inner(), v.to_string()))
                        .collect::<Vec<_>>(),
                    record.level(),
                    record.payload().to_string(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(check, from_sink);
    }

    #[test]
    fn kv_types() {
        struct Asserter;

        impl Sink for Asserter {
            fn should_log(&self, _: Level) -> bool {
                true
            }
            fn flush(&self) -> crate::Result<()> {
                Ok(())
            }
            fn level_filter(&self) -> LevelFilter {
                LevelFilter::All
            }
            fn set_level_filter(&self, _: LevelFilter) {
                unimplemented!()
            }
            fn set_formatter(&self, _: Box<dyn Formatter>) {
                unimplemented!()
            }
            fn set_error_handler(&self, _: Option<ErrorHandler>) {
                unimplemented!()
            }

            fn log(&self, record: &Record) -> crate::Result<()> {
                let kvs = record.key_values();
                let value = kvs.get(Key::from_str("v")).unwrap();
                assert_eq!(kvs.len(), 1);

                match record.payload() {
                    "1" => assert!(value.to_i64().is_some()),
                    "2" => assert!(value.to_str().is_some()),
                    "3" => assert!(value.to_i64().is_some()),
                    "4" => assert!(value.to_i64().is_some()),
                    "5" => assert!(value.is::<Vec<i32>>()),
                    "6" => assert!(value.is::<Data>()),
                    "7" => assert!(value.is::<Data>()),
                    _ => panic!(),
                }
                Ok(())
            }
        }

        let asserter = test_utils::build_test_logger(|b| b.sink(Arc::new(Asserter)));

        #[cfg_attr(feature = "sval", derive(sval_derive::Value))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        struct Data {
            i: i32,
            v: Vec<i32>,
        }
        let data = Data {
            i: 1,
            v: vec![1, 2],
        };

        info!(logger: asserter, "1", kv: { v = 1 });
        info!(logger: asserter, "2", kv: { v = "string" });
        info!(logger: asserter, "3", kv: { v: = 1 });
        info!(logger: asserter, "4", kv: { v:? = 1 });
        #[cfg(feature = "sval")]
        info!(logger: asserter, "5", kv: { v:sval = vec![1, 2] });
        #[cfg(feature = "sval")]
        info!(logger: asserter, "6", kv: { v:sval = data });
        #[cfg(feature = "serde")]
        info!(logger: asserter, "7", kv: { v:serde = data });
    }
}
