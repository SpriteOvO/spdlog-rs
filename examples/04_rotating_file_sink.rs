use std::{
    env,
    ffi::OsString,
    fs::{self, DirEntry},
    io,
    path::PathBuf,
    sync::Arc,
};

use spdlog::{
    prelude::*,
    sink::{RotatingFileSink, RotationPolicy},
};

fn main() {
    let logs_path: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("logs/rotating_file_sink");

    let path_by_size: PathBuf = logs_path.join("by_size.log");
    let path_hourly: PathBuf = logs_path.join("hourly.log");
    let path_daily: PathBuf = logs_path.join("daily.log");

    // See the documentation for descriptions.

    let by_size: Arc<RotatingFileSink> = Arc::new(
        RotatingFileSink::builder()
            .base_path(&path_by_size)
            .rotation_policy(RotationPolicy::FileSize(1024 * 10))
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );

    let hourly: Arc<RotatingFileSink> = Arc::new(
        RotatingFileSink::builder()
            .base_path(&path_hourly)
            .rotation_policy(RotationPolicy::Hourly)
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );

    let daily: Arc<RotatingFileSink> = Arc::new(
        RotatingFileSink::builder()
            .base_path(&path_daily)
            .rotation_policy(RotationPolicy::Daily { hour: 0, minute: 0 })
            .rotate_on_open(true)
            .build()
            .unwrap(),
    );

    let by_size: Logger = Logger::builder().sink(by_size).build();
    let hourly: Logger = Logger::builder().sink(hourly).build();
    let daily: Logger = Logger::builder().sink(daily).build();

    info!(logger: by_size, "hello, world");
    info!(logger: hourly, "hello, world");
    info!(logger: daily, "hello, world");

    info!(
        "log files: {:?}",
        fs::read_dir(logs_path)
            .unwrap()
            .collect::<Vec<io::Result<DirEntry>>>()
            .into_iter()
            .map(|p| p.unwrap().file_name())
            .collect::<Vec<OsString>>()
    );
}
