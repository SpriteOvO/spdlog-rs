use proc_macro2::Span;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Path, Token,
};

/// A parsed pattern.
///
/// A [`Pattern`] gives a structural representation of a pattern parsed from the
/// token stream given to the `pattern` macro.
pub(crate) struct Pattern {
    /// The template string included in the pattern.
    pub(crate) template: PatternTemplate,

    /// Any user-provided pattern-to-formatter mapping.
    pub(crate) custom_pat_mapping: CustomPatternMapping,
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template_literal: LitStr = input.parse()?;
        let template = PatternTemplate::parse_from_template(template_literal)?;

        input.parse::<Option<Token![,]>>()?;
        let custom_pat_mapping = input.parse()?;

        Ok(Self {
            template,
            custom_pat_mapping,
        })
    }
}

pub(crate) struct PatternTemplate {
    pub(crate) span: Span,
    pub(crate) tokens: Vec<PatternTemplateToken>,
}

impl PatternTemplate {
    fn parse_from_template(template: LitStr) -> syn::Result<Self> {
        todo!()
    }

    fn parse_from_str(s: &str) -> syn::Result<Self> {
        todo!()
    }
}

pub(crate) enum PatternTemplateToken {
    Literal(PatternTemplateLiteral),
    Formatter(PatternTemplateFormatter),
    ColorRange(PatternTemplate),
}

pub(crate) struct PatternTemplateLiteral {
    pub(crate) span: Span,
    pub(crate) literal: String,
}

pub(crate) struct PatternTemplateFormatter {
    pub(crate) span: Span,
    pub(crate) formatter_name: String,
}

/// Mapping from user-provided patterns to formatters.
pub(crate) struct CustomPatternMapping {
    pub(crate) mapping_pairs: Vec<(LitStr, CustomPatternFactoryFunctionId)>,
}

impl Parse for CustomPatternMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items: Punctuated<_, Token![,]> =
            input.parse_terminated(CustomPatternMappingItem::parse)?;

        let mut mapping_pairs = Vec::new();
        for i in items {
            for name in i.names {
                mapping_pairs.push((name, i.factory.clone()));
            }
        }

        Ok(Self { mapping_pairs })
    }
}

/// Identifier of a function that produces custom pattern formatters.
#[derive(Clone)]
pub(crate) struct CustomPatternFactoryFunctionId(pub(crate) Path);

impl From<Path> for CustomPatternFactoryFunctionId {
    fn from(p: Path) -> Self {
        Self(p)
    }
}

impl Parse for CustomPatternFactoryFunctionId {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let p = input.parse()?;
        Ok(Self(p))
    }
}

struct CustomPatternMappingItem {
    names: Vec<LitStr>,
    factory: CustomPatternFactoryFunctionId,
}

impl Parse for CustomPatternMappingItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let names_input;
        parenthesized!(names_input in input);

        let names: Punctuated<_, Token![,]> =
            names_input.parse_terminated(<LitStr as Parse>::parse)?;
        let names = names.into_iter().collect();

        input.parse::<Token![=>]>()?;

        let factory: CustomPatternFactoryFunctionId = input.parse()?;
        Ok(Self { names, factory })
    }
}
