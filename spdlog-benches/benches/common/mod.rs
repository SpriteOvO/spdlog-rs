use std::{
    env,
    fmt::Write,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    process::{self, Stdio},
    str,
    sync::LazyLock,
};

use spdlog::{
    prelude::*,
    sink::{GetSinkProp, Sink, SinkProp, WriteSink, WriteSinkBuilder},
    LoggerBuilder, Record,
};

#[allow(dead_code)]
pub static BENCH_LOGS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = Path::new(env!("OUT_DIR")).join("bench_logs");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
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
macro_rules! aggregate_bench_main {
    () => {
        fn main() {
            common::__aggregate_bench_main_impl(
                &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join(file!()),
            );
        }
    };
}

// Some log crates are based on `log` crate, which has only one global logger
// instance, meaning that the logger is only allowed to be configured once. In
// order to bench multiple different configurations, we need multiple child
// processes to bench, and this function is used as a launcher for those child
// processes.
pub fn __aggregate_bench_main_impl(source_file: &Path) {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 || args.get(1).unwrap() != "--bench" {
        eprintln!(
            "error: this is an aggregate bench and is supposed to be run only by `cargo bench`"
        );
        process::exit(1);
    }

    let current_exe = env::current_exe().unwrap();
    let current_dir = current_exe.parent().unwrap();

    let name = format!("{}_", env!("CARGO_CRATE_NAME"));

    let mut sub_benches = fs::read_dir(current_dir)
        .unwrap()
        .filter_map(|p| p.ok())
        .filter(|p| {
            #[cfg(unix)]
            let is_executable = p.path().metadata().is_ok_and(|m| {
                use std::os::unix::fs::PermissionsExt;
                m.is_file() && m.permissions().mode() & 0o111 != 0
            });
            #[cfg(windows)]
            let is_executable = p.path().extension().is_some_and(|ext| ext == "exe");

            p.file_name().to_string_lossy().starts_with(&name) && is_executable
        })
        .collect::<Vec<_>>();
    sub_benches.sort_by_key(|p| p.file_name());

    let mut sub_bench_sources = fs::read_dir(Path::new(source_file).parent().unwrap())
        .unwrap()
        .filter_map(|p| p.ok())
        .filter(|p| {
            p.file_name()
                .to_string_lossy()
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_digit())
                && p.path().extension().is_some_and(|ext| ext == "rs")
        })
        .collect::<Vec<_>>();
    sub_bench_sources.sort_by_key(|p| p.file_name());

    fn exit_as_files_mismatch() {
        eprintln!(
            "error: not all expected sub-benches have been built. try running `cargo bench` directly instead of specifying a `--bench` option."
        );
        process::exit(1);
    }
    if sub_benches.len() != sub_bench_sources.len() {
        exit_as_files_mismatch();
    }
    sub_bench_sources
        .into_iter()
        .zip(sub_benches.iter())
        .for_each(|(source, bin)| {
            let expected_start = format!(
                "{}_{}-",
                env!("CARGO_CRATE_NAME"),
                source.path().file_stem().unwrap().to_string_lossy()
            );
            if !bin
                .file_name()
                .to_string_lossy()
                .starts_with(&expected_start)
            {
                exit_as_files_mismatch();
            }
        });

    let mut captured_stdout = String::new();
    for sub_bench in sub_benches {
        let output = process::Command::new(sub_bench.path())
            .arg("--bench")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .unwrap();
        assert!(output.status.success());

        captured_stdout
            .write_str(str::from_utf8(&output.stdout).unwrap())
            .unwrap();
    }

    let results = captured_stdout
        .lines()
        .filter(|line| line.contains("test ") && line.contains(" ... "))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", results);
}

#[must_use]
pub fn build_bench_logger(cb: impl FnOnce(&mut LoggerBuilder) -> &mut LoggerBuilder) -> Logger {
    let mut builder = Logger::builder();
    cb(builder.error_handler(|err| panic!("{}", err)));
    builder.build().unwrap()
}

//

pub struct StringSink {
    underlying: WriteSink<Vec<u8>>,
}

impl StringSink {
    pub fn new() -> Self {
        Self {
            underlying: WriteSink::builder().target(vec![]).build().unwrap(),
        }
    }

    pub fn with(
        cb: impl FnOnce(
            WriteSinkBuilder<Vec<u8>, PhantomData<Vec<u8>>>,
        ) -> WriteSinkBuilder<Vec<u8>, PhantomData<Vec<u8>>>,
    ) -> Self {
        Self {
            underlying: cb(WriteSink::builder().target(vec![])).build().unwrap(),
        }
    }

    pub fn clone_string(&self) -> String {
        String::from_utf8(self.underlying.clone_target()).unwrap()
    }
}

impl GetSinkProp for StringSink {
    fn prop(&self) -> &SinkProp {
        self.underlying.prop()
    }
}

impl Sink for StringSink {
    fn log(&self, record: &Record) -> spdlog::Result<()> {
        self.underlying.log(record)
    }

    fn flush(&self) -> spdlog::Result<()> {
        self.underlying.flush()
    }
}
