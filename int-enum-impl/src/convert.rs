cfg_if::cfg_if! {
    if #[cfg(feature = "convert")] {
        pub use inner::convert_impls;
    } else {
        pub fn convert_impls(
            _: &syn::Ident,
            _: &syn::Path,
            _: &syn::Ident,
            _: &syn::Ident,
        ) -> proc_macro2::TokenStream {
            proc_macro2::TokenStream::new()
        }
    }
}

#[cfg(feature = "convert")]
mod inner {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse_quote;
    use syn::{Ident, ItemImpl, Path};

    pub fn convert_impls(crate_: &Ident, core: &Path, enum_: &Ident, int_: &Ident) -> TokenStream {
        let from_impls = from_impls(&crate_, &core, &enum_, &int_);
        let tryfrom_impl = tryfrom_impl(&crate_, &core, &enum_, &int_);

        quote! {
            #from_impls
            #tryfrom_impl
        }
    }

    fn from_impls(crate_: &Ident, core: &Path, enum_: &Ident, int_: &Ident) -> TokenStream {
        parse_quote! {
            impl #core::convert::From<#enum_> for #int_ {
                fn from(n: #enum_) -> Self {
                    #crate_::IntEnum::to_int(&n)
                }
            }

            impl #core::convert::From<&#enum_> for #int_ {
                fn from(n: &#enum_) -> Self {
                    #crate_::IntEnum::to_int(n)
                }
            }
        }
    }

    fn tryfrom_impl(crate_: &Ident, core: &Path, enum_: &Ident, int_: &Ident) -> ItemImpl {
        parse_quote! {
            impl #core::convert::TryFrom<#int_> for #enum_ {
                type Error = #crate_::IntEnumError<Self>;

                fn try_from(n: #int_) -> #core::result::Result<Self, Self::Error> {
                    #crate_::IntEnum::from_int(n)
                }
            }
        }
    }
}
