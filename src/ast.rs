use std::fmt;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, DataEnum, Expr, Fields, Meta, Token};

use crate::ext::SpanExt;
use crate::Result;

pub struct Repr {
    pub ident: Ident,
}

impl ToTokens for Repr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
    }
}

const VALID_REPRS: &[&str] =
    &["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "isize", "usize"];

pub fn get_repr(attrs: &[Attribute]) -> Result<Repr> {
    let mut repr_span = None::<Span>;

    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            let path = match meta {
                Meta::Path(path) => path,
                _ => continue,
            };
            let ident = match path.get_ident() {
                Some(ident) => ident,
                None => continue,
            };

            let sym = ident.to_string();
            let sym = sym.strip_prefix("r#").unwrap_or(sym.as_ref());

            if VALID_REPRS.contains(&sym) {
                return Ok(Repr { ident: ident.clone() });
            }
        }

        repr_span = Some(attr.span());
    }

    Err(match repr_span {
        Some(span) => span
            .resolved_at(Span::call_site())
            .error("missing type in `repr` attribute")
            .note(format!("valid reprs are {}", ValidReprs)),
        None => Span::call_site().error("missing `repr` attribute"),
    })
}

struct ValidReprs;

impl fmt::Display for ValidReprs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for repr in VALID_REPRS {
            if !first {
                f.write_str(", ")?;
            }
            first = false;

            f.write_str("`")?;
            f.write_str(repr)?;
            f.write_str("`")?;
        }

        Ok(())
    }
}

pub struct Variant {
    pub ident: Ident,
    pub discriminant: Expr,
}

pub fn get_variants(enum_ident: &Ident, data: DataEnum) -> Result<Vec<Variant>> {
    let mut variants = Vec::with_capacity(data.variants.len());
    let mut iter = data.variants.into_iter();

    let err_iter = loop {
        let v = match iter.next() {
            Some(next) => next,
            None => return Ok(variants),
        };

        let discriminant = match v.discriminant {
            Some((_, discriminant)) if matches!(v.fields, Fields::Unit) => discriminant,
            _ => break std::iter::once(v).chain(iter),
        };

        variants.push(Variant { ident: v.ident, discriminant });
    };

    let mut diag = enum_ident
        .span()
        .resolved_at(Span::call_site())
        .error("enum has variants that are not supported by this trait");

    for v in err_iter {
        if !matches!(&v.fields, Fields::Unit) {
            diag = diag.span_error(v.fields.span(), "only unit variants are supported");
        }
        if v.discriminant.is_none() {
            diag = diag.span_error(v.span().end(), "missing discriminant");
        }
    }

    Err(diag)
}
