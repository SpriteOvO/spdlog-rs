use std::sync::Arc;

use spdlog::{
    formatter::{pattern, PatternFormatter},
    prelude::*,
    sink::WriteSink,
};

#[cfg(feature = "log")]
#[test]
fn test_source_location() {
    let formatter = Box::new(PatternFormatter::new(pattern!(
        "({module_path}::{file_name}) {payload}{eol}"
    )));
    let sink = Arc::new(
        WriteSink::builder()
            .formatter(formatter)
            .target(Vec::new())
            .build()
            .unwrap(),
    );
    let logger = Arc::new(Logger::builder().sink(sink.clone()).build().unwrap());

    spdlog::init_log_crate_proxy().unwrap();
    spdlog::log_crate_proxy().set_logger(Some(logger));
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("text");
    assert_eq!(
        String::from_utf8(sink.clone_target()).unwrap(),
        "(log_crate_proxy::log_crate_proxy.rs) text\n"
    );
}
