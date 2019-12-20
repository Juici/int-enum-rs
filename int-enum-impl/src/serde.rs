cfg_if::cfg_if! {
    if #[cfg(feature = "serialize")] {
        pub use inner::serde_impls;
    } else {
        pub fn serde_impls(
            _: &syn::Ident,
            _: &syn::Path,
            _: &syn::Ident,
            _: &syn::Ident,
        ) -> proc_macro2::TokenStream {
            proc_macro2::TokenStream::new()
        }
    }
}

#[cfg(feature = "serialize")]
mod inner {
    use proc_macro2::{Span, TokenStream};
    use quote::quote;
    use syn::parse_quote;
    use syn::{Ident, ItemFn, ItemImpl, Path};

    pub fn serde_impls(crate_: &Ident, core: &Path, enum_: &Ident, int_: &Ident) -> TokenStream {
        let serde = parse_quote!(#crate_::__serde);

        let enum_str = enum_.to_string();
        let int_str = int_.to_string();

        let serialize_impl = serialize_impl(&crate_, &core, &serde, &enum_, &int_, &int_str);
        let deserialize_impl =
            deserialize_impl(&crate_, &core, &serde, &enum_, &int_, &enum_str, &int_str);

        quote! {
            #serialize_impl
            #deserialize_impl
        }
    }

    fn serialize_impl(
        crate_: &Ident,
        core: &Path,
        serde: &Path,
        enum_: &Ident,
        int_: &Ident,
        int_str: &str,
    ) -> ItemImpl {
        let ser_fn = Ident::new(&format!("serialize_{}", &int_str), Span::call_site());
        parse_quote! {
            impl #serde::Serialize for #enum_ {
                fn serialize<S>(&self, serializer: S) -> #core::result::Result<S::Ok, S::Error>
                where
                    S: #serde::Serializer
                {
                    let n: #int_ = #crate_::IntEnum::to_int(self);
                    serializer.#ser_fn(n)
                }
            }
        }
    }

    fn deserialize_impl(
        crate_: &Ident,
        core: &Path,
        serde: &Path,
        enum_: &Ident,
        int_: &Ident,
        enum_str: &str,
        int_str: &str,
    ) -> ItemImpl {
        let expecting = format!("{} integer", &int_str);
        let visit_fn = visit_fn(&crate_, &core, &serde, &int_, &enum_str, &int_str);

        parse_quote! {
            impl<'de> #serde::Deserialize<'de> for #enum_ {
                fn deserialize<D>(deserializer: D) -> #core::result::Result<Self, D::Error>
                where
                    D: #serde::Deserializer<'de>
                {
                    struct Visitor;

                    impl<'de> #serde::de::Visitor<'de> for Visitor {
                        type Value = #enum_;

                        fn expecting(
                            &self,
                            formatter: &mut #core::fmt::Formatter<'_>,
                        ) -> #core::fmt::Result {
                            formatter.write_str(#expecting)
                        }

                        #visit_fn
                    }

                    deserializer.deserialize_any(Visitor)
                }
            }
        }
    }

    fn visit_fn(
        crate_: &Ident,
        core: &Path,
        serde: &Path,
        int_: &Ident,
        enum_str: &str,
        int_str: &str,
    ) -> ItemFn {
        let unknown_value = {
            cfg_if::cfg_if! {
                if #[cfg(any(feature = "std", feature = "alloc"))] {
                    let unknown_value = format!("unknown {} value: {{}}", &enum_str);
                    quote!(custom(#crate_::__format!(#unknown_value, v)))
                } else {
                    let unknown_value = format!("unknown {} value", &enum_str);
                    quote!(custom(#unknown_value))
                }
            }
        };

        let (visit, fn_body) = match &int_str[..] {
            "i128" | "u128" => (
                quote!(#int_),
                quote!(#crate_::IntEnum::from_int(v as #int_).map_err(|_| E::#unknown_value)),
            ),
            s => {
                let (visit_fn, min_max, cond) = match s {
                    "i8" | "i16" | "i32" | "i64" | "isize" => (
                        quote!(i64),
                        quote! {
                            const MIN: i64 = #core::#int_::MIN as i64;
                            const MAX: i64 = #core::#int_::MAX as i64;
                        },
                        quote!(MIN <= v && v <= MAX),
                    ),
                    "u8" | "u16" | "u32" | "u64" | "usize" => (
                        quote!(u64),
                        quote! {
                            const MAX: u64 = #core::#int_::MAX as u64;
                        },
                        quote!(v <= MAX),
                    ),
                    _ => unreachable!(),
                };

                (
                    visit_fn,
                    quote! {
                        #min_max

                        if #cond {
                            let v: #core::result::Result<Self::Value, _> = #crate_::IntEnum::from_int(v as #int_);
                            if let #core::result::Result::Ok(v) = v {
                                return #core::result::Result::Ok(v);
                            }
                        }

                        #core::result::Result::Err(E::#unknown_value)
                    },
                )
            }
        };
        let fn_name = Ident::new(&format!("visit_{}", visit.to_string()), Span::call_site());

        parse_quote! {
            fn #fn_name<E>(self, v: #visit) -> #core::result::Result<Self::Value, E>
            where
                E: #serde::de::Error,
            {
                #fn_body
            }
        }
    }
}
