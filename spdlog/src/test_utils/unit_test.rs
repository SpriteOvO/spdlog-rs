// Test utils for unit tests only
//
// In this file, you can use public or private items from spdlog-rs as you wish,
// as they will be used from unit tests only.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::sync::*;

pub static TEST_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = Path::new(env!("OUT_DIR")).join("test_logs");
    _ = fs::create_dir(&path);
    path
});
