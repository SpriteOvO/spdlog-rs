//! Re-exports items from other crates for convenience.
//!
//! This module selectively re-exports items from other crates that may be used
//! by users. This effectively eliminates the hassle of manually adding
//! dependencies to the user's own `Cargo.toml`.
//!
//! Users can still call items from these crates directly, as long as semver
//! versions of the dependencies are compatible.

/// Items from [`log` crate].
///
/// The `log` crate has its own level filter, and logs produced by `log` crate
/// macros will first be filtered by `log` crate itself (this is not controlled
/// by `spdlog-rs`). When users enable the `log` crate compatibility layer
/// proxy, please make sure that `log` crate's own level filter is configured
/// appropriately so that `spdlog-rs` can receive logs.
///
/// - To enable the `log` crate compatibility layer proxy, call
///   [`spdlog::init_log_crate_proxy`].
///
/// - To configure `log` crate's own level filter, call
///   [`re_export::log::set_max_level`] with [`re_export::log::LevelFilter`].
///
/// [`log` crate]: https://docs.rs/log
/// [`spdlog::init_log_crate_proxy`]: crate::init_log_crate_proxy
/// [`re_export::log::set_max_level`]: crate::re_export::log::set_max_level
/// [`re_export::log::LevelFilter`]: crate::re_export::log::LevelFilter
#[cfg(feature = "log")]
pub mod log {
    pub use log::{set_max_level, LevelFilter, SetLoggerError};
}
