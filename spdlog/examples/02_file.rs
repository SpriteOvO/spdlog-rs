use std::{env, sync::Arc};

use spdlog::{
    prelude::*,
    sink::{FileSink, RotatingFileSink, RotationPolicy},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_file_logger()?;
    configure_rotating_daily_file_logger()?;
    configure_rotating_size_file_logger()?;
    configure_rotating_hourly_file_logger()?;
    configure_rotating_period_file_logger()?;

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

fn configure_rotating_daily_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("rotating_daily.log");

    let file_sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
            .build()?,
    );
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `rotating_daily.log`, and the file will be rotated daily at 00:00");

    Ok(())
}

fn configure_rotating_size_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("rotating_size.log");

    let file_sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::FileSize(1024))
            .build()?,
    );
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `rotating_size.log`, and the file will be rotated when its size reaches 1024 bytes");

    Ok(())
}

fn configure_rotating_hourly_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("rotating_hourly.log");

    let file_sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::Hourly)
            .build()?,
    );
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `rotating_hourly.log`, and the file will be rotated every hour");

    Ok(())
}

fn configure_rotating_period_file_logger() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_exe()?.with_file_name("rotating_period.log");

    let file_sink = Arc::new(
        RotatingFileSink::builder()
            .base_path(path)
            .rotation_policy(RotationPolicy::Period { duration: (chrono::Duration::hours(1) +
                                                                 chrono::Duration::minutes(2) +
                                                                 chrono::Duration::seconds(3)).to_std().unwrap() })
            .build()?,
    );
    let new_logger = Arc::new(Logger::builder().sink(file_sink).build()?);
    spdlog::set_default_logger(new_logger);

    info!("this log will be written to the file `rotating_period.log`, and the file will be rotated every hour, 2 minutes, and 3 seconds");

    Ok(())
}
