#[cfg(target_os = "android")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;

    use spdlog::{
        prelude::*,
        sink::{AndroidLogTag, AndroidSink},
    };

    let sink = Arc::new(
        AndroidSink::builder()
            .tag(AndroidLogTag::Custom("spdlog-rs-example".into()))
            .build()?,
    );
    let logger = spdlog::default_logger().fork_with(|logger| {
        logger.set_name(Some("demo")).unwrap();
        logger.sinks_mut().push(sink);
        Ok(())
    })?;
    spdlog::set_default_logger(logger);

    info!("info message from spdlog-rs's AndroidSink");
    error!("error message from spdlog-rs's AndroidSink", kv: { error_code = 114514 });
    Ok(())
}

#[cfg(not(target_os = "android"))]
fn main() {
    panic!("this example is only available on Android target");
}
