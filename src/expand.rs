use proc_macro2::{Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Variant};

use crate::ast::{EnumInput, Repr};
use crate::Result;

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { attrs, ident, generics, data, .. } = input;

    let error = match data {
        Data::Enum(data) => return derive_enum(EnumInput { attrs, ident, generics, data }),
        Data::Struct(_) => "this trait cannot be derived for structs",
        Data::Union(_) => "this trait cannot be derived for unions",
    };

    Err(Span::call_site().error(error))
}

fn derive_enum(input: EnumInput) -> Result<TokenStream> {
    verify_unit_variants(&input)?;

    let EnumInput { attrs, ident: enum_ident, generics, data, .. } = input;

    let repr = Repr::from_attributes(&attrs)?;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let from_int_arms = data.variants.iter().map(|Variant { ident, .. }| quote! {
        v if v == #enum_ident::#ident as #repr => ::core::result::Result::Ok(#enum_ident::#ident),
    });

    let from_enum_impl = quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::From<#enum_ident #type_generics> for #repr #where_clause {
            #[inline]
            fn from(v: #enum_ident #type_generics) -> Self {
                v as #repr
            }
        }
    };

    let try_from_int_impl = quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::TryFrom<#repr> for #enum_ident #type_generics #where_clause {
            type Error = #repr;

            #[inline]
            fn try_from(v: #repr) -> ::core::result::Result<Self, #repr> {
                match v {
                    #(#from_int_arms)*
                    v => ::core::result::Result::Err(v),
                }
            }
        }
    };

    Ok(quote! {
        #[allow(unknown_lints, non_local_definitions)] // False positive: https://github.com/rust-lang/rust/issues/121621
        const _: () = {
            #from_enum_impl
            #try_from_int_impl
        };
    })
}

fn verify_unit_variants(input: &EnumInput) -> Result<()> {
    let mut non_unit_variants =
        input.data.variants.iter().filter(|v| !matches!(v.fields, Fields::Unit));

    // First non-unit variant.
    let Some(first) = non_unit_variants.next() else { return Ok(()) };

    let mut diag = input
        .ident
        .span()
        .resolved_at(Span::call_site())
        .error("enum has variants that are not supported by this trait")
        .span_warning(first.fields.span(), "only unit variants are supported");

    // Add warnings for remaining non-unit variants.
    for v in non_unit_variants {
        diag = diag.span_warning(v.fields.span(), "only unit variants are supported");
    }

    Err(diag)
}
