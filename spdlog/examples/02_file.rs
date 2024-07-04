use std::{env, sync::Arc};

use spdlog::{
    prelude::*,
    sink::{FileSink, RotatingFileSink, RotationPolicy},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_file_logger()?;
    configure_rotating_file_logger()?;

    Ok(())
}

fn configure_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("file.log");

    let file_sink = Arc::new(FileSink::builder().path(path).build()?);
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `all.log`");

    Ok(())
}

fn configure_rotating_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("rotating.log");

    let file_sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
            .build()?,
    );
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `rotating.log`, and the file will be rotated daily at 00:00");

    Ok(())
}
