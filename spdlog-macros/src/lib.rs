//! Macros implementation of crate [`spdlog-rs`].
//!
//! `spdlog-macros` should currently not be used as a standalone dependency, use
//! [`spdlog-rs`] instead.
//!
//! [`spdlog-rs`]: https://crates.io/crates/spdlog-rs

mod pattern;

use proc_macro::TokenStream;

#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let pattern = syn::parse_macro_input!(input);

    match pattern::pattern_impl(pattern) {
        Ok(stream) => stream.into(),
        Err(err) => panic!("{}", err),
    }
}
