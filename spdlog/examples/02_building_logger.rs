use std::sync::Arc;

use spdlog::{prelude::*, sink::Sink};

fn main() -> Result<(), spdlog::Error> {
    // Building loggers
    //
    // A logger can combine multiple sinks, we'll explain how to build sinks in
    // later examples. Here we clone sinks of the default logger so that we can see
    // the output in our terminal.

    let sinks: Vec<Arc<dyn Sink>> = spdlog::default_logger().sinks().to_owned();
    let mut builder: LoggerBuilder = Logger::builder();
    let builder: &mut LoggerBuilder = builder.sinks(sinks).level_filter(LevelFilter::All);

    let gui: Logger = builder.name("gui").build()?;
    let network: Logger = builder.name("network").build()?;
    let settings: Logger = builder.name("settings").build()?;

    // Logging with our loggers instead of the default logger.

    info!(logger: gui, "user clicked 'check for updates'");

    info!(logger: network, "connect to the update server");
    warn!(logger: network, "failed to connect: {}. retry 1", "timeout");
    warn!(logger: network, "failed to connect: {}. retry 2", "timeout");
    info!(logger: network, "connection established");
    info!(logger: network, "fetched the latest version: {}", "v1.2.3");
    trace!(logger: network, "disconnect by client");

    info!(logger: gui, "ask user for the update");
    info!(logger: gui, "user clicked 'update now'");

    info!(logger: network, "connect to the download server");
    info!(logger: network, "connection established");
    info!(logger: network, "file size: {} bytes", 123456);
    info!(logger: network, "downloading...");
    trace!(logger: network, "downloading... {} %", 0);
    trace!(logger: network, "downloading... {} %", 25);
    trace!(logger: network, "downloading... {} %", 50);
    trace!(logger: network, "downloading... {} %", 75);
    trace!(logger: network, "downloading... {} %", 100);
    info!(logger: network, "download completed");
    trace!(logger: network, "disconnect by client");

    info!(
        logger: gui,
        "user selected 'check for updates' as '{}'", "daily"
    );

    info!(
        logger: settings,
        "change `updates_check_period` to {}s",
        24 * 60 * 60
    );
    info!(logger: settings, "settings saved");

    Ok(())
}
