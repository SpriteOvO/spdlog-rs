#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;

    use spdlog::{prelude::*, sink::WinDebugSink};

    let sink = Arc::new(WinDebugSink::builder().build()?);
    let logger = spdlog::default_logger().fork_with(|logger| {
        logger.set_name(Some("demo")).unwrap();
        logger.sinks_mut().push(sink);
        Ok(())
    })?;
    spdlog::set_default_logger(logger);

    info!("info message from spdlog-rs's WinDebugSink");
    error!("error message from spdlog-rs's WinDebugSink", kv: { error_code = 114514 });
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("this example is only available on Windows target");
}
