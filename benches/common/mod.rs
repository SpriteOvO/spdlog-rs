use std::{env, fs, path::PathBuf};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BENCH_LOGS_PATH: PathBuf = {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("dev/bench_logs");
        fs::create_dir_all(&path).unwrap();
        path
    };
}
