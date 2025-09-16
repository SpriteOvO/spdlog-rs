use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Expr, Ident, Token,
};

pub struct Normalize {
    callback: Ident,
    default_list: DefaultArgsList,
    args: Args,
}

impl Parse for Normalize {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let callback = input.parse::<Ident>()?;
        input.parse::<Token![=>]>()?;
        let default_list = DefaultArgsList::parse(input)?;
        input.parse::<Token![,]>()?;
        let args = Args::parse(input)?;
        Ok(Self {
            callback,
            default_list,
            args,
        })
    }
}

struct DefaultArgsList(Vec<Arg>);

impl Parse for DefaultArgsList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![default]>()?;
        let list;
        bracketed!(list in input);
        let list = list
            .parse_terminated(Arg::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self(list))
    }
}

struct Args(Vec<Arg>);

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.parse_terminated(Arg::parse, Token![,])?;
        Ok(Self(args.into_iter().collect()))
    }
}

enum Arg {
    Optional(OptionalArg),
    Other(ArgValue),
}

impl Arg {
    fn as_optional(&self) -> Option<&OptionalArg> {
        match self {
            Self::Optional(arg) => Some(arg),
            _ => None,
        }
    }

    fn as_optional_mut(&mut self) -> Option<&mut OptionalArg> {
        match self {
            Self::Optional(arg) => Some(arg),
            _ => None,
        }
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        match OptionalArg::parse(&fork) {
            Ok(opt_arg) => {
                input.advance_to(&fork);
                Ok(Self::Optional(opt_arg))
            }
            Err(_) => Ok(Self::Other(ArgValue::parse(input)?)),
        }
    }
}

struct OptionalArg {
    name: Ident,
    value: ArgValue,
}

impl Parse for OptionalArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let value = ArgValue::parse(input)?;
        Ok(Self { name, value })
    }
}

enum ArgValue {
    Expr(Expr),
    Braced(BraceAny),
}

impl ArgValue {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Self::Expr(expr) => expr.into_token_stream(),
            Self::Braced(braced) => braced.0,
        }
    }
}

impl Parse for ArgValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        match Expr::parse(&fork) {
            Ok(expr) => {
                input.advance_to(&fork);
                Ok(Self::Expr(expr))
            }
            Err(_) => Ok(BraceAny::parse(input).map(Self::Braced)?),
        }
    }
}

struct BraceAny(TokenStream);

impl Parse for BraceAny {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let ts: TokenStream = content.parse()?;
        Ok(Self(quote!({#ts})))
    }
}

fn check_inputs(normalize: &Normalize) -> syn::Result<()> {
    let mut seen_keys = HashSet::new();
    for arg in normalize.args.0.iter().filter_map(|arg| arg.as_optional()) {
        if !seen_keys.insert(&arg.name) {
            return Err(syn::Error::new(
                arg.name.span(),
                format!("found duplicate optional argument '{}'", arg.name),
            ));
        }
    }

    let groups = normalize
        .args
        .0
        .split_inclusive(|arg| matches!(arg, Arg::Optional(_)))
        .filter(|group| group.iter().any(|arg| !matches!(arg, Arg::Optional(_))))
        .collect::<Vec<_>>();
    if groups.len() > 1 {
        return Err(syn::Error::new(
            groups
                .first()
                .and_then(|group| group.last())
                .and_then(|arg| arg.as_optional())
                .unwrap()
                .name
                .span(),
            "optional arguments cannot occur in the middle of regular arguments",
        ));
    }

    Ok(())
}

pub fn normalize(normalize: Normalize) -> syn::Result<TokenStream> {
    check_inputs(&normalize)?;

    let mut default_args = normalize.default_list.0;
    let mut other_args = vec![];

    for input_arg in normalize.args.0 {
        match input_arg {
            Arg::Optional(input_arg) => {
                let stored = default_args
                    .iter_mut()
                    .find_map(|allowed| {
                        allowed
                            .as_optional_mut()
                            .and_then(|allowed| (allowed.name == input_arg.name).then_some(allowed))
                    })
                    .ok_or_else(|| {
                        syn::Error::new(
                            input_arg.name.span(),
                            format!("unknown optional parameter '{}'", input_arg.name),
                        )
                    })?;
                stored.value = input_arg.value;
            }
            Arg::Other(input_arg) => {
                other_args.push(input_arg);
            }
        }
    }

    let callback = normalize.callback;
    let default_args = default_args
        .into_iter()
        .map(|arg| match arg {
            Arg::Optional(arg) => {
                let name = arg.name;
                let value = arg.value.into_token_stream();
                quote!(#name: #value)
            }
            Arg::Other(arg) => {
                let value = arg.into_token_stream();
                quote!(#value)
            }
        })
        .collect::<Vec<_>>();
    let other_args = other_args
        .into_iter()
        .map(|arg| {
            let ts = arg.into_token_stream();
            quote!(#ts)
        })
        .collect::<Vec<_>>();

    let emitted = quote! {
        ::spdlog::#callback!(#(#default_args),*, #(#other_args),*)
    };
    Ok(emitted)
}
