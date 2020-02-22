use proc_macro2::TokenStream;

use crate::parse::IntEnum;

pub fn serde_impl(_input: &IntEnum) -> TokenStream {
    cfg_if::cfg_if! {
        if #[cfg(feature = "serde")] {
            inner::inner_impl(_input)
        } else {
            TokenStream::new()
        }
    }
}

#[cfg(feature = "serde")]
mod inner {
    use std::cmp::Ordering;

    use proc_macro2::{Span, TokenStream};
    use quote::quote;
    use syn::Ident;

    use crate::dummy;
    use crate::parse::IntEnum;

    pub fn inner_impl(input: &IntEnum) -> TokenStream {
        let enum_ty = &input.ident;
        let int_ty = &input.repr;

        let ser_impl = ser_impl(enum_ty, int_ty);
        let de_impl = de_impl(enum_ty, int_ty);

        let ser_block = dummy::wrap_in_const("Serialize", enum_ty, ser_impl);
        let de_block = dummy::wrap_in_const("Deserialize", enum_ty, de_impl);

        quote! {
            #ser_block
            #de_block
        }
    }

    fn ser_impl(enum_ty: &Ident, int_ty: &Ident) -> TokenStream {
        let serialize_fn = serde_fn("serialize", int_ty);

        quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl _int_enum::export::serde::Serialize for #enum_ty {
                fn serialize<S>(&self, serializer: S) -> _int_enum::export::Result<S::Ok, S::Error>
                where
                    S: _int_enum::export::serde::Serializer,
                {
                    let n: #int_ty = _int_enum::IntEnum::int_value(*self);

                    #serialize_fn
                }
            }
        }
    }

    fn de_impl(enum_ty: &Ident, int_ty: &Ident) -> TokenStream {
        let expecting_str = format!("{} integer", int_ty);

        let deserialize_fn = serde_fn("deserialize", int_ty);
        let visit_fns = visit_fns(int_ty);

        quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'de> _int_enum::export::serde::Deserialize<'de> for #enum_ty {
                fn deserialize<D>(deserializer: D) -> _int_enum::export::Result<Self, D::Error>
                where
                    D: _int_enum::export::serde::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> _int_enum::export::serde::de::Visitor<'de> for Visitor {
                        type Value = #enum_ty;

                        fn expecting(
                            &self,
                            f: &mut _int_enum::export::fmt::Formatter,
                        ) -> _int_enum::export::fmt::Result {
                            f.write_str(#expecting_str)
                        }

                        #visit_fns
                    }

                    #deserialize_fn
                }
            }
        }
    }

    fn serde_fn(serde: &str, int_ty: &Ident) -> TokenStream {
        let int_ty_str = int_ty.to_string();

        macro_rules! serde_fn {
            ($apparent_int_ty:expr) => {{
                let fn_ident = Ident::new(
                    &format!("{}_{}", serde, $apparent_int_ty),
                    Span::call_site(),
                );

                match serde {
                    "serialize" => {
                        let size_ident = Ident::new(
                            &format!("{}", $apparent_int_ty),
                            Span::call_site(),
                        );
                        quote!( serializer.#fn_ident(n as #size_ident) )
                    }
                    "deserialize" => quote!( deserializer.#fn_ident(Visitor) ),
                    _ => unreachable!(),
                }
            }};
        }

        macro_rules! size_fns {
            (@($ty:literal)) => {{
                let serde_16 = serde_fn!(concat!($ty, "16"));
                let serde_32 = serde_fn!(concat!($ty, "32"));
                let serde_64 = serde_fn!(concat!($ty, "64"));

                quote! {
                    #[cfg(target_pointer_width = "16")]
                    {
                        #serde_16
                    }

                    #[cfg(target_pointer_width = "32")]
                    {
                        #serde_32
                    }

                    #[cfg(target_pointer_width = "64")]
                    {
                        #serde_64
                    }
                }
            }};
            (usize) => { size_fns!(@("u")) };
            (isize) => { size_fns!(@("i")) };
        }

        match &int_ty_str[..] {
            "usize" => size_fns!(usize),
            "isize" => size_fns!(isize),
            _ => serde_fn!(int_ty),
        }
    }

    fn visit_fns(int_ty: &Ident) -> TokenStream {
        let int_ty_str = int_ty.to_string();

        match &int_ty_str[0..1] {
            "u" => visit_fns_uint(int_ty, &int_ty_str),
            "i" => visit_fns_int(int_ty, &int_ty_str),
            _ => unreachable!(),
        }
    }

    // TODO: Clean up visit fns.

    fn visit_fns_uint(int_ty: &Ident, int_ty_str: &str) -> TokenStream {
        match &int_ty_str[..] {
            "u8" | "u16" | "u32" | "u64" | "u128" => {
                let visit_u8 = visit_fn(int_ty, int_ty_str, "u8");
                let visit_u16 = visit_fn(int_ty, int_ty_str, "u16");
                let visit_u32 = visit_fn(int_ty, int_ty_str, "u32");
                let visit_u64 = visit_fn(int_ty, int_ty_str, "u64");
                let visit_u128 = visit_fn(int_ty, int_ty_str, "u128");

                quote! {
                    #visit_u8
                    #visit_u16
                    #visit_u32
                    #visit_u64
                    #visit_u128
                }
            }
            "usize" => {
                let mut visit_fns = TokenStream::new();

                for size in &["u16", "u32", "u64"] {
                    let visit_u8 = visit_fn(int_ty, size, "u8");
                    let visit_u16 = visit_fn(int_ty, size, "u16");
                    let visit_u32 = visit_fn(int_ty, size, "u32");
                    let visit_u64 = visit_fn(int_ty, size, "u64");
                    let visit_u128 = visit_fn(int_ty, size, "u128");

                    let size = &size[1..];

                    visit_fns = quote! {
                        #visit_fns

                        #[cfg(target_pointer_width = #size)]
                        #visit_u8

                        #[cfg(target_pointer_width = #size)]
                        #visit_u16

                        #[cfg(target_pointer_width = #size)]
                        #visit_u32

                        #[cfg(target_pointer_width = #size)]
                        #visit_u64

                        #[cfg(target_pointer_width = #size)]
                        #visit_u128
                    }
                }

                visit_fns
            }
            _ => unreachable!(),
        }
    }

    fn visit_fns_int(int_ty: &Ident, int_ty_str: &str) -> TokenStream {
        match &int_ty_str[..] {
            "i8" | "i16" | "i32" | "i64" | "i128" => {
                let visit_i8 = visit_fn(int_ty, int_ty_str, "i8");
                let visit_i16 = visit_fn(int_ty, int_ty_str, "i16");
                let visit_i32 = visit_fn(int_ty, int_ty_str, "i32");
                let visit_i64 = visit_fn(int_ty, int_ty_str, "i64");
                let visit_i128 = visit_fn(int_ty, int_ty_str, "i128");

                let visit_u8 = visit_fn(int_ty, int_ty_str, "u8");
                let visit_u16 = visit_fn(int_ty, int_ty_str, "u16");
                let visit_u32 = visit_fn(int_ty, int_ty_str, "u32");
                let visit_u64 = visit_fn(int_ty, int_ty_str, "u64");
                let visit_u128 = visit_fn(int_ty, int_ty_str, "u128");

                quote! {
                    #visit_i8
                    #visit_i16
                    #visit_i32
                    #visit_i64
                    #visit_i128

                    #visit_u8
                    #visit_u16
                    #visit_u32
                    #visit_u64
                    #visit_u128
                }
            }
            "isize" => {
                let mut visit_fns = TokenStream::new();

                for size in &["i16", "i32", "i64"] {
                    let visit_i8 = visit_fn(int_ty, size, "i8");
                    let visit_i16 = visit_fn(int_ty, size, "i16");
                    let visit_i32 = visit_fn(int_ty, size, "i32");
                    let visit_i64 = visit_fn(int_ty, size, "i64");
                    let visit_i128 = visit_fn(int_ty, size, "i128");

                    let visit_u8 = visit_fn(int_ty, size, "u8");
                    let visit_u16 = visit_fn(int_ty, size, "u16");
                    let visit_u32 = visit_fn(int_ty, size, "u32");
                    let visit_u64 = visit_fn(int_ty, size, "u64");
                    let visit_u128 = visit_fn(int_ty, size, "u128");

                    let size = &size[1..];

                    visit_fns = quote! {
                        #visit_fns

                        #[cfg(target_pointer_width = #size)]
                        #visit_i8

                        #[cfg(target_pointer_width = #size)]
                        #visit_i16

                        #[cfg(target_pointer_width = #size)]
                        #visit_i32

                        #[cfg(target_pointer_width = #size)]
                        #visit_i64

                        #[cfg(target_pointer_width = #size)]
                        #visit_i128

                        #[cfg(target_pointer_width = #size)]
                        #visit_u8

                        #[cfg(target_pointer_width = #size)]
                        #visit_u16

                        #[cfg(target_pointer_width = #size)]
                        #visit_u32

                        #[cfg(target_pointer_width = #size)]
                        #visit_u64

                        #[cfg(target_pointer_width = #size)]
                        #visit_u128
                    }
                }

                visit_fns
            }
            _ => unreachable!(),
        }
    }

    fn visit_fn(int_ty: &Ident, apparent_int_ty: &str, visit_ty_str: &str) -> TokenStream {
        let visit_ty = Ident::new(visit_ty_str, Span::call_site());

        let body = if visit_ty_str == apparent_int_ty
            || (visit_ty_str[0..1] == apparent_int_ty[0..1]
                && ord_int(visit_ty_str, apparent_int_ty) == Ordering::Less)
        {
            // Visit type can fit inside int type.
            quote! {
                match _int_enum::IntEnum::from_int(v as #int_ty) {
                    Ok(ret) => Ok(ret),
                    Err(err) => Err(E::custom(err)),
                }
            }
        } else {
            // Visit type is larger than int type and may overflow.
            let apparent_int_ty_ident = Ident::new(apparent_int_ty, Span::call_site());
            let visit_int_ty = Ident::new(&format!("visit_{}", apparent_int_ty), Span::call_site());

            let min_max = match (&apparent_int_ty[0..1], &visit_ty_str[0..1]) {
                ("i", "u") => quote! {
                    const MIN: #visit_ty = 0;
                    const MAX: #visit_ty = #int_ty::max_value() as #visit_ty;
                },
                (_, _) => quote! {
                    const MIN: #visit_ty = #int_ty::min_value() as #visit_ty;
                    const MAX: #visit_ty = #int_ty::max_value() as #visit_ty;
                },
            };

            quote! {
                #min_max

                if MIN <= v && v <= MAX {
                    self.#visit_int_ty(v as #apparent_int_ty_ident)
                } else {
                    Err(E::custom(format_args!(
                        "unknown variant `{}`, out of range [{}, {}]",
                        v, MIN, MAX,
                    )))
                }
            }
        };

        let visit_fn = Ident::new(&format!("visit_{}", visit_ty_str), Span::call_site());

        quote! {
            fn #visit_fn<E>(self, v: #visit_ty) -> Result<Self::Value, E>
            where
                E: _int_enum::export::serde::de::Error,
            {
                #body
            }
        }
    }

    // Order int types without parsing.
    fn ord_int(a: &str, b: &str) -> Ordering {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    }
}
