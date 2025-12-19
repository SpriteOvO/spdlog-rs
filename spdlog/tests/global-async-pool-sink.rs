use std::{
    env,
    fmt::Write,
    os::raw::c_int,
    process::{self, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use spdlog::{
    prelude::*,
    sink::{AsyncPoolSink, GetSinkProp, Sink, SinkProp},
};

static IS_LOGGED: AtomicBool = AtomicBool::new(false);
static IS_FLUSHED: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
struct SetFlagSink {
    prop: SinkProp,
}

impl GetSinkProp for SetFlagSink {
    fn prop(&self) -> &SinkProp {
        &self.prop
    }
}

impl Sink for SetFlagSink {
    fn log(&self, _record: &spdlog::Record) -> error::Result<()> {
        IS_LOGGED.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&self) -> error::Result<()> {
        // Assert that the record has been logged before flushing
        assert!(IS_LOGGED.load(Ordering::SeqCst));
        IS_FLUSHED.store(true, Ordering::SeqCst);
        Ok(())
    }
}

fn run_test() {
    {
        extern "C" fn check() {
            // Assert that `AsyncPoolSink` in the default logger will be flushed correctly
            // and will not panic.
            assert!(IS_FLUSHED.load(Ordering::SeqCst));
        }
        // Setup `atexit` to check the flag at the end of the program
        extern "C" {
            fn atexit(cb: extern "C" fn()) -> c_int;
        }
        assert_eq!(unsafe { atexit(check) }, 0);

        let async_pool_sink = AsyncPoolSink::builder()
            .sink(Arc::new(SetFlagSink::default()))
            .build_arc()
            .unwrap();
        let logger = Logger::builder()
            .sink(async_pool_sink)
            .level_filter(LevelFilter::All)
            .flush_level_filter(LevelFilter::Off)
            .build_arc()
            .unwrap();
        spdlog::set_default_logger(logger);
    }

    info!("hello async_pool_sink");
}

fn main() {
    // https://github.com/SpriteOvO/spdlog-rs/issues/64

    // This is a flaky test, it only has a certain probability of failing, so we run
    // it multiple times to make sure it's really working properly.
    {
        let mut captured_output = String::new();
        let args = env::args().collect::<Vec<_>>();
        // If this is the parent process (no additional arguments)
        if args.len() == 1 {
            for i in 0..1000 {
                let output = process::Command::new(&args[0])
                    .arg("child")
                    .stderr(Stdio::piped())
                    .output()
                    .unwrap();
                let success = output.status.success();

                writeln!(
                    captured_output,
                    "Attempt #{i} = {}",
                    if success { "ok" } else { "failed!" }
                )
                .unwrap();

                if !success {
                    eprintln!("{captured_output}");

                    let stderr = String::from_utf8_lossy(&output.stderr).lines().fold(
                        String::new(),
                        |mut contents, line| {
                            writeln!(&mut contents, "> {line}").unwrap();
                            contents
                        },
                    );
                    eprintln!("stderr of the failed attempt:\n{stderr}");

                    panic!("Test failed");
                }
            }
            return;
        } else {
            assert_eq!(args[1], "child");
        }

        // Run the test after leaving the scope, so the main function ends
        // without dropping additional variables, thus exiting faster. This
        // should increase the probability of reproducing the error.
    }
    run_test();
}
