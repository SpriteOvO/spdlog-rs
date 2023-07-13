// Test utils for unit tests only
//
// In this file, you can use public or private items from spdlog-rs as you wish,
// as they will be used from unit tests only.

use std::{env, fs, path::PathBuf};

use crate::sync::*;

pub static TEST_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("dev/test_logs");
    fs::create_dir_all(&path).unwrap();
    path
});
