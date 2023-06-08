mod synthesis;

use proc_macro2::TokenStream;
use self_cell::self_cell;
use spdlog_internal::pattern_parser::{parse::Template, PatternRegistry, Result};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Ident, LitStr, Path, Token,
};
use synthesis::Synthesiser;

pub fn pattern_impl(pattern: Pattern) -> Result<TokenStream> {
    let mut registry = PatternRegistry::with_builtin();
    for (name, formatter) in pattern.custom_patterns() {
        registry.register_custom(name.to_string(), formatter.clone())?;
    }

    Synthesiser::new(registry).synthesize(pattern.template())
}

/// A parsed pattern.
///
/// A [`Pattern`] gives a structural representation of a pattern parsed from the
/// token stream given to the `pattern` macro.

pub struct Pattern {
    /// The template string included in the pattern.
    template: TemplateSelfRef,
    /// Any user-provided pattern-to-formatter mapping.
    custom_patterns: CustomPatterns,
}

self_cell! {
    pub struct TemplateSelfRef {
        owner: String,
        #[covariant]
        dependent: Template,
    }
}

impl Pattern {
    fn custom_patterns(&self) -> impl IntoIterator<Item = &(Ident, Path)> {
        self.custom_patterns.0.iter()
    }

    fn template(&self) -> &Template {
        self.template.borrow_dependent()
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template_lit = input.parse::<LitStr>()?;
        input.parse::<Option<Token![,]>>()?;
        let custom_patterns = input.parse()?;

        let ret = Pattern {
            template: TemplateSelfRef::try_new(template_lit.value(), |template_str| {
                Template::parse(template_str).map_err(|err| {
                    syn::Error::new(
                        // TODO: Maybe we can make a subspan for the literal for a better error
                        // message
                        template_lit.span(),
                        err,
                    )
                })
            })?,
            custom_patterns,
        };
        Ok(ret)
    }
}

/////

/// Mapping from user-provided patterns to formatters.
struct CustomPatterns(Vec<(Ident, Path)>);

impl Parse for CustomPatterns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items = input.parse_terminated(CustomPatternItem::parse, Token![,])?;

        let mapping_pairs = items
            .into_iter()
            .map(|item| (item.name, item.factory))
            .collect();

        Ok(Self(mapping_pairs))
    }
}

struct CustomPatternItem {
    name: Ident,
    factory: Path,
}

impl Parse for CustomPatternItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_input;
        braced!(name_input in input);
        name_input.parse::<Token![$]>()?;
        let name = name_input.parse()?;
        input.parse::<Token![=>]>()?;
        let factory = input.parse()?;

        Ok(Self { name, factory })
    }
}
