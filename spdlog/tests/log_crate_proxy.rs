use std::sync::Arc;

use spdlog::formatter::{pattern, PatternFormatter};

include!(concat!(
    env!("OUT_DIR"),
    "/test_utils/common_for_integration_test.rs"
));
use test_utils::*;

#[cfg(feature = "log")]
#[test]
fn test_source_location() {
    let formatter = Box::new(PatternFormatter::new(pattern!(
        "({module_path}::{file_name}) {payload}{eol}"
    )));
    let sink = Arc::new(StringSink::with(|b| b.formatter(formatter)));
    let logger = Arc::new(build_test_logger(|b| b.sink(sink.clone())));

    spdlog::init_log_crate_proxy().unwrap();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("text");
    assert_eq!(
        sink.clone_string(),
        "(log_crate_proxy::log_crate_proxy.rs) text\n"
    );
}
