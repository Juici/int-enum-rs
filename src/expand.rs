use proc_macro2::{Ident, Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::quote;
use syn::{Attribute, Data, DataEnum, DeriveInput, Generics, Visibility};

use crate::ast::Variant;
use crate::{ast, Result};

pub struct EnumInput {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: DataEnum,
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { attrs, vis, ident, generics, data } = input;

    let error = match data {
        Data::Enum(data) => return derive_enum(EnumInput { attrs, vis, ident, generics, data }),
        Data::Struct(_) => "this trait cannot be derived for structs",
        Data::Union(_) => "this trait cannot be derived for unions",
    };

    Err(Span::call_site().error(error))
}

fn derive_enum(input: EnumInput) -> Result<TokenStream> {
    let EnumInput { attrs, ident: enum_ident, generics, data, .. } = input;

    let repr = ast::get_repr(&attrs)?;
    let variants = ast::get_variants(&enum_ident, data)?;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let from_enum_args = variants.iter().map(|Variant { ident, discriminant, .. }| {
        quote! {
            #enum_ident::#ident => #discriminant,
        }
    });

    let try_from_int_args = variants.iter().map(|Variant { discriminant, ident, .. }| {
        quote! {
            v if v == (#discriminant) => ::core::result::Result::Ok(#enum_ident::#ident),
        }
    });

    let from_enum_impl = quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::From<#enum_ident #type_generics> for #repr #where_clause {
            #[inline]
            fn from(v: #enum_ident #type_generics) -> Self {
                match v {
                    #(#from_enum_args)*
                }
            }
        }
    };

    let try_from_int_impl = quote! {
        #[automatically_derived]
        impl #impl_generics ::core::convert::TryFrom<#repr> for #enum_ident #type_generics #where_clause {
            type Error = #repr;

            #[inline]
            fn try_from(v: #repr) -> ::core::result::Result<Self, Self::Error> {
                match v {
                    #(#try_from_int_args)*
                    v => ::core::result::Result::Err(v),
                }
            }
        }
    };

    Ok(quote! {
        #from_enum_impl
        #try_from_int_impl
    })
}
