use proc_macro2::TokenStream;
use quote::quote;

use crate::dummy;
use crate::parse::IntEnum;

pub fn int_enum_impl(input: &IntEnum) -> TokenStream {
    let enum_ty = &input.ident;
    let int_ty = &input.repr;

    let to_int_arms = input.variants.iter().map(|(variant, value)| {
        quote! {
            #enum_ty::#variant => { #value }
        }
    });

    let from_int_arms = input.variants.iter().map(|(variant, value)| {
        quote! {
            #value => { _int_enum::export::Result::Ok(#enum_ty::#variant) }
        }
    });

    let impl_block = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl _int_enum::IntEnum for #enum_ty {
            type Int = #int_ty;

            #[inline]
            fn int_value(self) -> Self::Int {
                match self {
                    #(#to_int_arms)*
                }
            }

            #[inline]
            fn from_int(n: Self::Int) -> _int_enum::export::Result<Self, _int_enum::IntEnumError<Self>>
            where
                Self: Sized,
            {
                match n {
                    #(#from_int_arms)*
                    _ => { _int_enum::export::Result::Err(_int_enum::IntEnumError::__new(n)) }
                }
            }
        }
    };

    dummy::wrap_in_const("IntEnum", enum_ty, impl_block)
}
