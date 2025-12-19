use std::borrow::Cow;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens as _;
use spdlog_internal::pattern_parser::{
    error::TemplateError,
    parse::{Template, TemplateFormatterToken, TemplateLiteral, TemplateToken},
    Error, PatternKind as GenericPatternKind, PatternRegistry as GenericPatternRegistry, Result,
};
use syn::{Expr, ExprLit, Lit, LitStr, Path};

type PatternRegistry = GenericPatternRegistry<Path>;
type PatternKind = GenericPatternKind<Path>;

pub(crate) struct Synthesiser {
    registry: PatternRegistry,
}

impl Synthesiser {
    pub fn new(registry: PatternRegistry) -> Self {
        Self { registry }
    }

    pub fn synthesize(&self, template: &Template) -> Result<TokenStream> {
        let expr = self.build_expr(template, false)?;
        Ok(expr.into_token_stream())
    }

    fn build_expr(&self, template: &Template, mut style_range_seen: bool) -> Result<Expr> {
        let mut tuple_elems = Vec::with_capacity(template.tokens.len());

        for token in &template.tokens {
            let token_template_expr = match token {
                TemplateToken::Literal(literal_token) => self.build_literal(literal_token)?,
                TemplateToken::Formatter(formatter_token) => {
                    self.build_formatter_creation(formatter_token)?
                }
                TemplateToken::StyleRange(style_range_token) => {
                    if style_range_seen {
                        return Err(Error::Template(TemplateError::MultipleStyleRange));
                    }
                    style_range_seen = true;
                    let nested_pattern = self.build_expr(&style_range_token.body, true)?;
                    self.build_style_range_creation(nested_pattern)?
                }
            };
            tuple_elems.push(token_template_expr);
        }

        let stream = quote::quote! { ( #(#tuple_elems ,)* ) };
        let expr = syn::parse2(stream).unwrap();
        Ok(Expr::Tuple(expr))
    }

    fn build_literal(&self, literal_token: &TemplateLiteral) -> Result<Expr> {
        let lit = LitStr::new(&literal_token.literal, Span::mixed_site());
        let expr = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Str(lit),
        });
        Ok(expr)
    }

    fn build_formatter_creation(&self, formatter_token: &TemplateFormatterToken) -> Result<Expr> {
        let pattern = self.registry.find(
            formatter_token.has_custom_prefix,
            formatter_token.placeholder,
        )?;

        let factory = factory_of_pattern(pattern);
        let stream = quote::quote!( #factory() );
        let factory_call = syn::parse2(stream).unwrap();
        Ok(Expr::Call(factory_call))
    }

    fn build_style_range_creation(&self, body: Expr) -> Result<Expr> {
        let style_range_pattern_new_path: Path =
            syn::parse_str("::spdlog::formatter::__pattern::StyleRange::new").unwrap();
        let stream = quote::quote!( #style_range_pattern_new_path (#body) );
        let expr = syn::parse2(stream).unwrap();
        Ok(Expr::Call(expr))
    }
}

pub(crate) fn factory_of_pattern(pattern: &PatternKind) -> Cow<'_, Path> {
    match pattern {
        PatternKind::BuiltIn(builtin) => Cow::Owned(
            syn::parse_str::<Path>(&format!(
                "::spdlog::formatter::__pattern::{}::default",
                builtin.struct_name()
            ))
            .unwrap(),
        ),
        PatternKind::Custom { factory, .. } => Cow::Borrowed(factory),
    }
}
