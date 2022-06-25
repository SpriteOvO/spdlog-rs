mod parse;
mod synthesis;

use proc_macro::TokenStream;

use crate::parse::Pattern;
use crate::synthesis::Synthesiser;

/// Build a pattern formatter from a compile-time pattern string.
///
/// # Usage
///
/// In it's simplest form, `pattern` receives a **literal** pattern string and
/// converts it into a zero-cost pattern formatter:
///
/// ```ignore
/// let formatter = pattern!("pattern string");
/// ```
#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let pat = syn::parse_macro_input!(input as Pattern);

    let mut synthesiser = Synthesiser::with_builtin_formatters();
    for (name, formatter) in pat.custom_pat_mapping.mapping_pairs {
        if let Err(err) = synthesiser.add_formatter_mapping(name.value(), formatter.0) {
            panic!("{}", err);
        }
    }

    match synthesiser.synthesis(&pat.template) {
        Ok(stream) => stream.into(),
        Err(err) => panic!("{}", err),
    }
}
