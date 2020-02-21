use proc_macro2::TokenStream;

use crate::parse::IntEnum;

pub fn convert_impl(_input: &IntEnum) -> TokenStream {
    cfg_if::cfg_if! {
        if #[cfg(feature = "convert")] {
            inner_impl(_input)
        } else {
            TokenStream::new()
        }
    }
}

#[cfg(feature = "convert")]
fn inner_impl(input: &IntEnum) -> TokenStream {
    use quote::quote;

    use crate::dummy;

    let enum_ty = &input.ident;
    let int_ty = &input.repr;

    let from_enum = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl _int_enum::export::From<#enum_ty> for #int_ty {
            #[inline]
            fn from(n: #enum_ty) -> Self {
                _int_enum::IntEnum::int_value(n)
            }
        }
    };

    let try_from_int = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl _int_enum::export::TryFrom<#int_ty> for #enum_ty {
            type Error = _int_enum::IntEnumError<Self>;

            #[inline]
            fn try_from(n: #int_ty) -> _int_enum::export::Result<Self, Self::Error> {
                _int_enum::IntEnum::from_int(n)
            }
        }
    };

    let impl_blocks = quote! {
        #from_enum
        #try_from_int
    };

    dummy::wrap_in_const("CONVERT", enum_ty, impl_blocks)
}
