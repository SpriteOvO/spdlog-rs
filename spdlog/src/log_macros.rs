/// Logs a message at the specified level.
///
/// This macro will generically log with the specified [`Level`] and `format!`
/// based argument list.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::{log, Level};
///
/// # let app_events = spdlog::default_logger();
/// let data = (42, "Forty-two");
///
/// // Using the global default logger
/// log!(Level::Info, "received data: {}, {}", data.0, data.1);
///
/// // Or using the specified logger, and structured logging
/// log!(logger: app_events, Level::Info, "received data", kv: { data:? });
/// ```
///
/// [`Level`]: crate::Level
#[macro_export]
macro_rules! log {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE], $($input)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_impl {
    (logger: $logger:expr, kv: $kv:tt, then: $then:ident, $level:expr, $($arg:tt)+) => ({
        let logger = &$logger;
        if $crate::STATIC_LEVEL_FILTER.__test_const($level) && logger.should_log($level) {
            $crate::__log(logger, $level, $crate::source_location_current!(), $crate::__kv!($kv), format_args!($($arg)+));
        }
        $crate::__then_impl!($then, $($arg)+)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __then_impl {
    (__DISABLE, $($arg:tt)+) => {{}};
    ($then:ident, $($arg:tt)+) => {{
        $then!($($arg)+)
    }};
}

/// Logs a message at the critical level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::critical;
///
/// # let app_events = spdlog::default_logger();
/// let (left, right) = (true, false);
///
/// // Using the global default logger
/// critical!("runtime assertion failed. Left: `{}`, Right: `{}`", left, right);
///
/// // Or using the specified logger, and structured logging
/// critical!(logger: app_events, "runtime assertion failed.", kv: { left, right });
/// ```
#[macro_export]
macro_rules! critical {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Critical], $($input)+)
    };
}

/// Logs a message at the error level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::error;
///
/// # let app_events = spdlog::default_logger();
/// let (err_info, port) = ("No connection", 22);
///
/// // Using the global default logger
/// error!("error: {} on port {}", err_info, port);
///
/// // Or using the specified logger, and structured logging
/// error!(logger: app_events, "app error", kv: { reason = err_info, port });
/// ```
#[macro_export]
macro_rules! error {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Error], $($input)+)
    };
}

/// Logs a message at the warn level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
/// # Examples
///
/// ```
/// use spdlog::warn;
///
/// # let input_events = spdlog::default_logger();
/// let warn_description = "Invalid Input";
///
/// // Using the global default logger
/// warn!("warning! {}!", warn_description);
///
/// // Or using the specified logger, and structured logging
/// warn!(logger: input_events, "app received warning", kv: { reason = warn_description });
/// ```
#[macro_export]
macro_rules! warn {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Warn], $($input)+)
    };
}

/// Logs a message at the info level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
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
/// info!("connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
///
/// // Or using the specified logger, and structured logging
/// info!(logger: conn_events, "successfull connection", kv: { port = conn_info.port, speed = conn_info.speed });
/// ```
#[macro_export]
macro_rules! info {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Info], $($input)+)
    };
}

/// Logs a message at the debug level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
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
/// debug!("new position: x: {}, y: {}", pos.x, pos.y);
///
/// // Or using the specified logger, and structured logging
/// debug!(logger: app_events, "new position", kv: { x = pos.x, y = pos.y });
/// ```
#[macro_export]
macro_rules! debug {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Debug], $($input)+)
    };
}

/// Logs a message at the trace level.
#[doc = include_str!("./include/doc/log-macro-named-opt-params.md")]
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
/// trace!("position is: x: {}, y: {}", pos.x, pos.y);
///
/// // Or using the specified logger, and structured logging
/// trace!(logger: app_events, "position updated", kv: { x = pos.x, y = pos.y });
/// ```
#[macro_export]
macro_rules! trace {
    ($($input:tt)+) => {
        $crate::__normalize_forward!(__log_impl => default[logger: $crate::default_logger(), kv: {}, then: __DISABLE, $crate::Level::Trace], $($input)+)
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
        let (int, mod1, mod2) = (114514, Mods, Mods);

        const LEVEL_ARG: Level = Level::Info;

        let assert_records = |kv: &[(&'static str, &str)], payload| {
            let records = test_sink.records();
            assert_eq!(records.len(), Level::count() + 1);
            test_sink.clear();

            records
                .into_iter()
                .zip([
                    LEVEL_ARG,
                    Level::Trace,
                    Level::Debug,
                    Level::Info,
                    Level::Warn,
                    Level::Error,
                    Level::Critical,
                ])
                .for_each(|(record, expected_level)| {
                    assert_eq!(record.level(), expected_level);
                    assert_eq!(
                        record
                            .key_values()
                            .into_iter()
                            .map(|(k, v)| (k.inner(), v.to_string()))
                            .collect::<Vec<_>>(),
                        kv.iter()
                            .map(|(k, v)| (KeyInner::StaticStr(k), v.to_string()))
                            .collect::<Vec<_>>()
                    );
                    assert_eq!(record.payload(), payload);
                });
        };

        log!(logger: test, LEVEL_ARG, "logger param only");
        trace!(logger: test, "logger param only");
        debug!(logger: test, "logger param only");
        info!(logger: test, "logger param only");
        warn!(logger: test, "logger param only");
        error!(logger: test, "logger param only");
        critical!(logger: test, "logger param only");
        assert_records(&[], "logger param only");

        log!(logger: test, kv: {}, LEVEL_ARG, "empty kv param");
        trace!(logger: test, kv: {}, "empty kv param");
        debug!(logger: test, kv: {}, "empty kv param");
        info!(logger: test, kv: {}, "empty kv param");
        warn!(logger: test, kv: {}, "empty kv param");
        error!(logger: test, kv: {}, "empty kv param");
        critical!(logger: test, kv: {}, "empty kv param");
        assert_records(&[], "empty kv param");

        log!(logger: test, kv: { int = 114514 }, LEVEL_ARG, "kv capture value directly");
        trace!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        debug!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        info!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        warn!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        error!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        critical!(logger: test, kv: { int = 114514 }, "kv capture value directly");
        assert_records(&[("int", "114514")], "kv capture value directly");

        log!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, LEVEL_ARG, "kv capture value using modifiers");
        trace!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        debug!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        info!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        warn!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        error!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        critical!(logger: test, kv: { int = 114514, mod1: = Mods, mod2:? = Mods }, "kv capture value using modifiers");
        assert_records(
            &[("int", "114514"), ("mod1", "display"), ("mod2", "debug")],
            "kv capture value using modifiers",
        );

        log!(logger: test, kv: { int }, LEVEL_ARG, "kv shorthand");
        trace!(logger: test, kv: { int }, "kv shorthand");
        debug!(logger: test, kv: { int }, "kv shorthand");
        info!(logger: test, kv: { int }, "kv shorthand");
        warn!(logger: test, kv: { int }, "kv shorthand");
        error!(logger: test, kv: { int }, "kv shorthand");
        critical!(logger: test, kv: { int }, "kv shorthand");
        assert_records(&[("int", "114514")], "kv shorthand");

        log!(logger: test, kv: { int, mod1:, mod2:? }, LEVEL_ARG, "kv shorthand modifiers");
        trace!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        debug!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        info!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        warn!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        error!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        critical!(logger: test, kv: { int, mod1:, mod2:? }, "kv shorthand modifiers");
        assert_records(
            &[("int", "114514"), ("mod1", "display"), ("mod2", "debug")],
            "kv shorthand modifiers",
        );

        log!(logger: test, LEVEL_ARG, "params arbitrary order: logger, format, kv", kv: { mod1: });
        trace!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        debug!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        info!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        warn!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        error!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        critical!(logger: test, "params arbitrary order: logger, format, kv", kv: { mod1: });
        assert_records(
            &[("mod1", "display")],
            "params arbitrary order: logger, format, kv",
        );

        log!(LEVEL_ARG, "params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        trace!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        debug!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        info!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        warn!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        error!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        critical!("params arbitrary order = format, kv, logger", kv: { mod1:? }, logger: test);
        assert_records(
            &[("mod1", "debug")],
            "params arbitrary order = format, kv, logger",
        );

        let runtime_level = Level::Info;
        log!(logger: test, runtime_level, "runtime level");
    }

    #[test]
    #[should_panic(expected = "log then panic")]
    fn then_param() {
        error!("log then {}", "panic", then: panic);
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
