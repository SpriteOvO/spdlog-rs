//! Macros implementation of crate [`spdlog-rs`].
//!
//! `spdlog-macros` should currently not be used as a standalone dependency, use
//! [`spdlog-rs`] instead.
//!
//! [`spdlog-rs`]: https://crates.io/crates/spdlog-rs

mod pattern;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use spdlog_internal::pattern_parser::Result;

#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let pattern = syn::parse_macro_input!(input);
    into_or_error(pattern::pattern_impl(pattern))
}

#[proc_macro]
pub fn runtime_pattern(input: TokenStream) -> TokenStream {
    // We must make this macro a procedural macro because macro cannot match the "$"
    // token which is used in the custom patterns.

    let runtime_pattern = syn::parse_macro_input!(input);
    into_or_error(pattern::runtime_pattern_impl(runtime_pattern))
}

fn into_or_error(result: Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(stream) => stream.into(),
        Err(err) => panic!("{}", err),
    }
}
