use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{DeriveInput, Path, Result};

use crate::ast::Input;
use crate::{serde, util};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;

    util::with_crate_path(|int_enum| {
        let mut output = base(int_enum, &input);

        if cfg!(feature = "convert") {
            output.append_all(convert(int_enum, &input));
        }
        if cfg!(feature = "serde") {
            output.append_all(serde::serde_impl(int_enum, &input)?);
        }

        Ok(output)
    })
}

pub fn base(int_enum: &Path, input: &Input) -> TokenStream {
    let enum_ty = &input.ident;
    let int_ty = &input.repr;

    let to_int_arms = input.variants.iter().map(|(variant, value)| {
        quote! {
            #enum_ty::#variant => { #value }
        }
    });

    let from_int_arms = input.variants.iter().map(|(variant, value)| {
        quote! {
            #value => { #int_enum::__private::Result::Ok(#enum_ty::#variant) }
        }
    });

    quote! {
        #[automatically_derived]
        impl #int_enum::IntEnum for #enum_ty {
            type Int = #int_ty;

            #[inline]
            fn int_value(self) -> Self::Int {
                match self {
                    #(#to_int_arms)*
                }
            }

            #[inline]
            fn from_int(n: Self::Int) -> #int_enum::__private::Result<Self, #int_enum::IntEnumError<Self>>
            where
                Self: Sized,
            {
                match n {
                    #(#from_int_arms)*
                    _ => { #int_enum::__private::Result::Err(#int_enum::IntEnumError::__new(n)) }
                }
            }
        }
    }
}

fn convert(int_enum: &Path, input: &Input) -> TokenStream {
    let enum_ty = &input.ident;
    let int_ty = &input.repr;

    let from_enum = quote! {
        #[automatically_derived]
        impl #int_enum::__private::From<#enum_ty> for #int_ty {
            #[inline]
            fn from(n: #enum_ty) -> Self {
                #int_enum::IntEnum::int_value(n)
            }
        }
    };

    let try_from_int = quote! {
        #[automatically_derived]
        impl #int_enum::__private::TryFrom<#int_ty> for #enum_ty {
            type Error = #int_enum::IntEnumError<Self>;

            #[inline]
            fn try_from(n: #int_ty) -> #int_enum::__private::Result<Self, Self::Error> {
                #int_enum::IntEnum::from_int(n)
            }
        }
    };

    quote! {
        #from_enum
        #try_from_int
    }
}
