use std::{env, fs, path::PathBuf};

use once_cell::sync::Lazy;

pub static BENCH_LOGS_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("dev/bench_logs");
    fs::create_dir_all(&path).unwrap();
    path
});

#[macro_export]
macro_rules! bench_log_message {
    () => {
        "this is a test log message"
    };
}

// These values are shared in Rust crate benchmarks.
// Benchmark "compare_with_cpp_spdlog" defines its own values in its file.

#[allow(dead_code)]
pub const FILE_SIZE: u64 = 30 * 1024 * 1024;

#[allow(dead_code)]
pub const ROTATING_FILES: usize = 6;

#[macro_export]
macro_rules! unavailable_bench {
    ( $($bench_name:ident),*$(,)? ) => {
        $(
            #[bench]
            #[ignore]
            fn $bench_name(_bencher: &mut Bencher) {}
        )*
    };
}

#[macro_export]
macro_rules! required_multi_thread_feature {
    () => {
        #[cfg(not(feature = "multi-thread"))]
        compile_error!("please rerun `cargo bench` with `--features multi-thread`");
    };
}
