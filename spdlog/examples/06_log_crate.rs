fn main() -> Result<(), Box<dyn std::error::Error>> {
    spdlog::init_log_crate_proxy()
        .expect("users should only call `init_log_crate_proxy` function once");

    // Setup filter as needed.
    let filter = env_filter::Builder::new().try_parse("RUST_LOG")?.build();
    spdlog::log_crate_proxy().set_filter(Some(filter));

    log::set_max_level(log::LevelFilter::Trace);
    log::trace!("this log will be processed by the global default logger in spdlog-rs");

    let custom_logger = spdlog::default_logger().fork_with_name(Some("another_logger"))?;
    spdlog::log_crate_proxy().set_logger(Some(custom_logger));
    log::info!("this log will be processed by custom_logger in spdlog-rs");

    spdlog::log_crate_proxy().set_logger(None);
    log::trace!("this log will be processed by the global default logger in spdlog-rs");

    Ok(())
}
