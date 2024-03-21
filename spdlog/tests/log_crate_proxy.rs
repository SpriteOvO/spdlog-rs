use std::sync::{Arc, Mutex};

use spdlog::formatter::{pattern, PatternFormatter};

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

static GLOBAL_LOG_CRATE_PROXY_MUTEX: Mutex<()> = Mutex::new(());

#[cfg(feature = "log")]
#[test]
fn test_source_location() {
    let formatter = Box::new(PatternFormatter::new(pattern!(
        "({module_path}::{file_name}) {payload}{eol}"
    )));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    let _guard = GLOBAL_LOG_CRATE_PROXY_MUTEX.lock().unwrap();
    spdlog::init_log_crate_proxy().ok();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("text");
    assert_eq!(
        sink.clone_string(),
        "(log_crate_proxy::log_crate_proxy.rs) text\n"
    );
}

#[cfg(feature = "log")]
#[test]
fn test_target() {
    let formatter = Box::new(PatternFormatter::new(pattern!("[{logger}] {payload}{eol}")));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    let _guard = GLOBAL_LOG_CRATE_PROXY_MUTEX.lock().unwrap();
    spdlog::init_log_crate_proxy().ok();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!(target: "MyLogger", "body");
    assert_eq!(sink.clone_string(), "[MyLogger] body\n");
}
