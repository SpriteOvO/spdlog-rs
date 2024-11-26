//! Macros implementation of crate [`spdlog-rs`].
//!
//! `spdlog-macros` should currently not be used as a standalone dependency, use
//! [`spdlog-rs`] instead.
//!
//! [`spdlog-rs`]: https://crates.io/crates/spdlog-rs

mod normalize_forward;
mod pattern;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let pattern = syn::parse_macro_input!(input);
    into_or_error(pattern::pattern_impl(pattern).map_err(Error::PatternParser))
}

#[proc_macro]
pub fn runtime_pattern(input: TokenStream) -> TokenStream {
    // We must make this macro a procedural macro because macro cannot match the "$"
    // token which is used in the custom patterns.

    let runtime_pattern = syn::parse_macro_input!(input);
    into_or_error(pattern::runtime_pattern_impl(runtime_pattern).map_err(Error::PatternParser))
}

// Example:
//
// ```rust
// normalize_forward!(callback => default[opt1: 1, opt2: {}, opt3: { 3 }, d], opt1: 10, a, b, c, opt3: { 30 });
// // will be converted to
// spdlog::callback!(opt1: 10, opt2: {}, opt3: { 30 }, d, a, b, c);
// ```
#[proc_macro]
pub fn normalize_forward(input: TokenStream) -> TokenStream {
    let normalize = syn::parse_macro_input!(input);
    into_or_error(normalize_forward::normalize(normalize).map_err(Error::NormalizeForward))
}

enum Error {
    PatternParser(spdlog_internal::pattern_parser::Error),
    NormalizeForward(syn::Error),
}

impl Error {
    fn emit(self) -> TokenStream2 {
        match self {
            Error::PatternParser(err) => {
                let error = err.to_string();
                quote!(compile_error!(#error))
            }
            Error::NormalizeForward(err) => err.to_compile_error(),
        }
    }
}

fn into_or_error(result: Result<TokenStream2, Error>) -> TokenStream {
    match result {
        Ok(stream) => stream.into(),
        Err(err) => err.emit().into(),
    }
}
