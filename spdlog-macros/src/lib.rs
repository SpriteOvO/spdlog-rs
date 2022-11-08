mod helper;
mod parse;
mod synthesis;

use proc_macro::TokenStream;

use crate::{
    parse::Pattern,
    synthesis::{PatternFormatter, PatternFormatterKind, Synthesiser},
};

#[proc_macro]
pub fn pattern(input: TokenStream) -> TokenStream {
    let pat = syn::parse_macro_input!(input as Pattern);

    let mut synthesiser = Synthesiser::with_builtin_formatters();
    for (name, formatter) in pat.custom_pat_mapping.mapping_pairs {
        if let Err(err) = synthesiser.add_formatter_mapping(
            name.to_string(),
            PatternFormatter {
                factory_path: formatter.0,
                kind: PatternFormatterKind::Custom,
            },
        ) {
            panic!("{}", err);
        }
    }

    match synthesiser.synthesis(&pat.template) {
        Ok(stream) => stream.into(),
        Err(err) => panic!("{}", err),
    }
}
