use std::sync::Arc;

use spdlog::prelude::*;

// Assuming that this function comes from an upstream dependency, it internally
// uses log crate to output logs.
fn fn_from_other_crate() {
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("this is a log from other crate");
}

fn main() {
    // Call this function early. Logs from log crate will not be handled before
    // calling it.
    spdlog::init_log_crate_proxy()
        .expect("users should only call `init_log_crate_proxy` function once");

    // Logs will be output to `spdlog::default_logger()`.
    fn_from_other_crate();

    // Assuming this is a custom logger, it might be a combination of
    // `StdStreamSink` and `FileSink` or whatever.
    let custom_logger: Arc<Logger> = spdlog::default_logger();

    // Logs will be output to `custom_logger`.
    let proxy: &'static spdlog::LogCrateProxy = spdlog::log_crate_proxy();
    proxy.set_logger(Some(custom_logger));
    fn_from_other_crate();

    // Logs will be output to `spdlog::default_logger()`.
    proxy.set_logger(None);
    fn_from_other_crate();
}
