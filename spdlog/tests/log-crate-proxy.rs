use std::sync::{Arc, Mutex};

use spdlog::{
    formatter::{pattern, PatternFormatter},
    __EOL,
};

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

static GLOBAL_LOG_CRATE_PROXY_MUTEX: Mutex<()> = Mutex::new(());

#[cfg(feature = "log")]
#[test]
fn test_source_location() {
    let formatter = PatternFormatter::new(pattern!("({module_path}::{file_name}) {payload}{eol}"));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    let _guard = GLOBAL_LOG_CRATE_PROXY_MUTEX.lock().unwrap();
    _ = spdlog::init_log_crate_proxy();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("text");
    assert_eq!(
        sink.clone_string(),
        format!("(log_crate_proxy::log-crate-proxy.rs) text{__EOL}")
    );
}

#[cfg(feature = "log")]
#[test]
fn test_target() {
    let formatter = PatternFormatter::new(pattern!("[{logger}] {payload}{eol}"));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    let _guard = GLOBAL_LOG_CRATE_PROXY_MUTEX.lock().unwrap();
    _ = spdlog::init_log_crate_proxy();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!(target: "MyLogger", "body");
    assert_eq!(sink.clone_string(), format!("[MyLogger] body{__EOL}"));
}

#[cfg(feature = "log")]
#[test]
fn test_kv() {
    let formatter = PatternFormatter::new(pattern!("{payload} {{ {kv} }}{eol}"));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    let _guard = GLOBAL_LOG_CRATE_PROXY_MUTEX.lock().unwrap();
    _ = spdlog::init_log_crate_proxy();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!(key1 = 42, key2 = true; "a {} event", "log");
    assert_eq!(
        sink.clone_string(),
        format!("a log event {{ key1=42 key2=true }}{__EOL}")
    );
}
