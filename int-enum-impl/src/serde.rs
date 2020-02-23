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
    use proc_macro2::{Span, TokenStream};
    use quote::quote;
    use syn::{Attribute, Ident, ItemFn};

    use crate::dummy;
    use crate::parse::{IntEnum, IntSize, IntStyle, IntType};

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

    fn ser_impl(enum_ty: &Ident, int_ty: &IntType) -> TokenStream {
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

    fn de_impl(enum_ty: &Ident, int_ty: &IntType) -> TokenStream {
        let expecting_str = format!("{} integer", int_ty);

        let deserialize_fn = serde_fn("deserialize", int_ty);
        let visit_fns = visit_fns(int_ty, int_ty.size);

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

                        #(#visit_fns)*
                    }

                    #deserialize_fn
                }
            }
        }
    }

    // `serialize_{}` or `deserialize_{}` function with respect to ptr size.
    fn serde_fn(serde: &str, int_ty: &IntType) -> TokenStream {
        // TODO: Clean up this function.

        macro_rules! serde_fn {
            ($actual_int_ty:expr) => {{
                let fn_ident = Ident::new(
                    &format!("{}_{}", serde, $actual_int_ty),
                    Span::call_site(),
                );

                match serde {
                    "serialize" => {
                        let size_ident = Ident::new(
                            &format!("{}", $actual_int_ty), // Duck-typed, forced into str.
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

        match int_ty.size {
            IntSize::_size => match int_ty.style {
                IntStyle::Signed => size_fns!(isize),
                IntStyle::Unsigned => size_fns!(usize),
            },
            _ => serde_fn!(int_ty),
        }
    }

    const SIZES: [IntSize; 5] = [
        IntSize::_8,
        IntSize::_16,
        IntSize::_32,
        IntSize::_64,
        IntSize::_128,
    ];
    const PTR_SIZES: [IntSize; 3] = [IntSize::_16, IntSize::_32, IntSize::_64];

    fn visit_fns(int_ty: &IntType, actual_int_size: IntSize) -> Vec<ItemFn> {
        if actual_int_size == IntSize::_size {
            return visit_fns_ptr_size(int_ty);
        }

        // Signed `int_ty` uses both signed and unsigned functions.
        let mut fns = Vec::with_capacity(match int_ty.style {
            IntStyle::Signed => SIZES.len() * 2,
            IntStyle::Unsigned => SIZES.len(),
        });

        for &visit_size in SIZES.iter() {
            fns.push(visit_fn(int_ty, actual_int_size, int_ty.style, visit_size));

            // `serde_json` will not give a positive integer to a signed visit fn, even if
            // it would fit.
            if int_ty.style == IntStyle::Signed {
                fns.push(visit_fn(
                    int_ty,
                    actual_int_size,
                    IntStyle::Unsigned,
                    visit_size,
                ));
            }
        }

        fns
    }

    fn visit_fns_ptr_size(int_ty: &IntType) -> Vec<ItemFn> {
        const ALLOC_LEN: usize = SIZES.len() * PTR_SIZES.len();

        // Allocate enough to hold all functions.
        let mut all_fns = Vec::with_capacity(match int_ty.style {
            IntStyle::Signed => ALLOC_LEN * 2,
            IntStyle::Unsigned => ALLOC_LEN,
        });

        for &ptr_size in PTR_SIZES.iter() {
            let mut fns = visit_fns(int_ty, ptr_size);

            // Add `#[cfg(target_pointer_width = "...")]` attributes.
            let width = ptr_size.to_string();
            let attr: Attribute = syn::parse_quote!( #[cfg(target_pointer_width = #width)] );

            for f in &mut fns {
                f.attrs.push(attr.clone());
            }

            all_fns.append(&mut fns);
        }

        all_fns
    }

    fn visit_fn(
        int_ty: &IntType,
        actual_int_size: IntSize,
        visit_ty_style: IntStyle,
        visit_ty_size: IntSize,
    ) -> ItemFn {
        let visit_ty = Ident::new(
            &format!("{}{}", visit_ty_style, visit_ty_size),
            Span::call_site(),
        );

        let body = if int_ty.style == visit_ty_style && visit_ty_size <= actual_int_size {
            // Visit type can fit inside int type.

            quote! {
                match _int_enum::IntEnum::from_int(v as #int_ty) {
                    _int_enum::export::Result::Ok(ret) => _int_enum::export::Result::Ok(ret),
                    _int_enum::export::Result::Err(err) => _int_enum::export::Result::Err(E::custom(err)),
                }
            }
        } else {
            // Visit type is larger than int type and may overflow.

            let (bounds, cond) = match (int_ty.style, visit_ty_style) {
                // If visiting unsigned int but fitting a signed int.
                (IntStyle::Signed, IntStyle::Unsigned) => {
                    // Here our `MIN` bound is not used in the condition, since an unsigned value
                    // cannot be negative and thus can't be smaller than the `MIN` bound.
                    //
                    // Instead our `MIN` bound is of `int_ty` and is only used in the error message.
                    let bounds = quote! {
                        const MIN: #int_ty = #int_ty::min_value();
                        const MAX: #visit_ty = #int_ty::max_value() as #visit_ty;
                    };
                    let cond = quote!(v <= MAX);
                    (bounds, cond)
                }
                _ => {
                    let bounds = quote! {
                        const MIN: #visit_ty = #int_ty::min_value() as #visit_ty;
                        const MAX: #visit_ty = #int_ty::max_value() as #visit_ty;
                    };
                    let cond = quote!(MIN <= v && v <= MAX);
                    (bounds, cond)
                }
            };

            let actual_int_ty = Ident::new(
                &format!("{}{}", int_ty.style, actual_int_size),
                Span::call_site(),
            );
            let visit_fn = Ident::new(&format!("visit_{}", actual_int_ty), Span::call_site());

            quote! {
                #bounds

                if #cond {
                    self.#visit_fn(v as #actual_int_ty)
                } else {
                    _int_enum::export::Result::Err(E::custom(_int_enum::export::format_args!(
                        "unknown variant `{}`, out of range [{}, {}]",
                        v, MIN, MAX,
                    )))
                }
            }
        };

        let visit_fn = Ident::new(&format!("visit_{}", visit_ty), Span::call_site());
        syn::parse_quote! {
            fn #visit_fn<E>(self, v: #visit_ty) -> Result<Self::Value, E>
            where
                E: _int_enum::export::serde::de::Error,
            {
                #body
            }
        }
    }
}
