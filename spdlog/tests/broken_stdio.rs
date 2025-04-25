// https://github.com/SpriteOvO/spdlog-rs/discussions/87
//
// Rust's print macros will panic if a write fails, we should avoid using
// print macros internally.
fn main() {
    #[cfg(target_family = "unix")]
    {
        let dev_full = std::ffi::CString::new("/dev/full").unwrap();
        unsafe {
            let fd = libc::open(dev_full.as_ptr(), libc::O_WRONLY);
            libc::dup2(fd, libc::STDOUT_FILENO);
            libc::dup2(fd, libc::STDERR_FILENO);
        }
    }
    // TODO: Other platforms?
    spdlog::info!("will panic if print macros are used internally");
    spdlog::error!("will panic if print macros are used internally");
}
