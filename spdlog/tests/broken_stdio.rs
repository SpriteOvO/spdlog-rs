// https://github.com/SpriteOvO/spdlog-rs/discussions/87
//
// Rust's print macros will panic if a write fails, we should avoid using
// print macros internally.

#[cfg(all(target_os = "linux", not(miri)))]
fn main() {
    #[cfg(not(feature = "std-stream-captured"))]
    run(); // Should not panic

    // Expect this test to panic when the "std-stream-captured" feature is enabled,
    // because we intentionally use print macros in `StdStreamSink` for capturing
    // output for `cargo test` and/or `cargo bench`.
    #[cfg(feature = "std-stream-captured")]
    assert!(std::panic::catch_unwind(run).is_err());

    fn run() {
        {
            let dev_full = std::ffi::CString::new("/dev/full").unwrap();
            unsafe {
                let fd = libc::open(dev_full.as_ptr(), libc::O_WRONLY);
                libc::dup2(fd, libc::STDOUT_FILENO);
                libc::dup2(fd, libc::STDERR_FILENO);
            }
        }
        spdlog::info!("will panic if print macros are used internally");
        spdlog::error!("will panic if print macros are used internally");
    }
}

#[cfg(any(
    not(target_os = "linux"),
    miri // Miri does not support closing std stream.
))]
fn main() {
    // TODO: Other platforms?
}
