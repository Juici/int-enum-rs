use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Path, Result};

use crate::ast::{Input, IntSize, IntStyle, Repr};

pub fn serde_impl(int_enum: &Path, input: &Input) -> Result<TokenStream> {
    let enum_ty = &input.ident;
    let int_ty = &input.repr;

    if int_ty.size == IntSize::Size {
        return Err(Error::new(
            int_ty.ident.span(),
            format_args!("serde is not supported for `{}`", int_ty),
        ));
    }

    let ser_impl = ser_impl(int_enum, enum_ty, int_ty);
    let de_impl = de_impl(int_enum, enum_ty, int_ty);

    Ok(quote! {
        #ser_impl
        #de_impl
    })
}

fn ser_impl(int_enum: &Path, enum_ty: &Ident, int_ty: &Repr) -> TokenStream {
    let serialize_fn = serde_fn("serialize", int_ty);

    quote! {
        #[automatically_derived]
        impl #int_enum::__private::serde::Serialize for #enum_ty {
            fn serialize<S>(&self, serializer: S) -> #int_enum::__private::Result<S::Ok, S::Error>
            where
                S: #int_enum::__private::serde::Serializer,
            {
                let n: #int_ty = #int_enum::IntEnum::int_value(*self);

                #serialize_fn
            }
        }
    }
}

fn de_impl(int_enum: &Path, enum_ty: &Ident, int_ty: &Repr) -> TokenStream {
    let expecting_str = format!("{} integer", int_ty);

    let deserialize_fn = serde_fn("deserialize", int_ty);
    let visit_fns = visit_fns(int_enum, int_ty, int_ty.size);

    quote! {
        #[automatically_derived]
        impl<'de> #int_enum::__private::serde::Deserialize<'de> for #enum_ty {
            fn deserialize<D>(deserializer: D) -> #int_enum::__private::Result<Self, D::Error>
            where
                D: #int_enum::__private::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> #int_enum::__private::serde::de::Visitor<'de> for Visitor {
                    type Value = #enum_ty;

                    fn expecting(
                        &self,
                        f: &mut #int_enum::__private::fmt::Formatter,
                    ) -> #int_enum::__private::fmt::Result {
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
fn serde_fn(serde: &str, int_ty: &Repr) -> TokenStream {
    let fn_ident = format_ident!("{}_{}", serde, int_ty);

    match serde {
        "serialize" => {
            quote!( serializer.#fn_ident(n as #int_ty) )
        }
        "deserialize" => quote!( deserializer.#fn_ident(Visitor) ),
        _ => unreachable!(),
    }
}

const SIZES: [IntSize; 5] = [IntSize::_8, IntSize::_16, IntSize::_32, IntSize::_64, IntSize::_128];

fn visit_fns(int_enum: &Path, int_ty: &Repr, actual_int_size: IntSize) -> Vec<TokenStream> {
    // Signed `int_ty` uses both signed and unsigned functions.
    let mut fns = Vec::with_capacity(match int_ty.style {
        IntStyle::Signed => SIZES.len() * 2,
        IntStyle::Unsigned => SIZES.len(),
    });

    for visit_size in SIZES {
        fns.push(visit_fn(int_enum, int_ty, actual_int_size, int_ty.style, visit_size));

        // `serde_json` will not give a positive integer to a signed visit fn, even if
        // it would fit.
        if int_ty.style == IntStyle::Signed {
            fns.push(visit_fn(int_enum, int_ty, actual_int_size, IntStyle::Unsigned, visit_size));
        }
    }

    fns
}

fn visit_fn(
    int_enum: &Path,
    int_ty: &Repr,
    actual_int_size: IntSize,
    visit_ty_style: IntStyle,
    visit_ty_size: IntSize,
) -> TokenStream {
    let visit_ty = format_ident!("{}{}", visit_ty_style, visit_ty_size);

    let body = if int_ty.style == visit_ty_style && visit_ty_size <= actual_int_size {
        // Visit type can fit inside int type.

        quote! {
            match #int_enum::IntEnum::from_int(v as #int_ty) {
                #int_enum::__private::Result::Ok(ret) => #int_enum::__private::Result::Ok(ret),
                #int_enum::__private::Result::Err(err) => #int_enum::__private::Result::Err(E::custom(err)),
            }
        }
    } else {
        // Visit type is larger than int type and may overflow.

        let (bounds, cond) = match (int_ty.style, visit_ty_style) {
            // Visiting unsigned int but fitting a signed int.
            (IntStyle::Signed, IntStyle::Unsigned) => {
                // Here our `MIN` bound is not used in the condition, since an unsigned value
                // cannot be negative and thus can't be smaller than the `MIN` bound.
                //
                // Instead our `MIN` bound is of `int_ty` and is only used in the error message.
                let bounds = quote! {
                    const MIN: #int_ty = #int_ty::MIN;
                    const MAX: #visit_ty = #int_ty::MAX as #visit_ty;
                };
                let cond = quote!(v <= MAX);
                (bounds, cond)
            }
            _ => {
                let bounds = quote! {
                    const MIN: #visit_ty = #int_ty::MIN as #visit_ty;
                    const MAX: #visit_ty = #int_ty::MAX as #visit_ty;
                };
                let cond = quote!(MIN <= v && v <= MAX);
                (bounds, cond)
            }
        };

        let actual_int_ty = format_ident!("{}{}", int_ty.style, actual_int_size);
        let visit_fn = format_ident!("visit_{}", actual_int_ty);

        quote! {
            #bounds

            if #cond {
                self.#visit_fn(v as #actual_int_ty)
            } else {
                #int_enum::__private::Result::Err(E::custom(#int_enum::__private::format_args!(
                    "unknown variant `{}`, out of range [{}, {}]",
                    v, MIN, MAX,
                )))
            }
        }
    };

    let visit_fn = format_ident!("visit_{}", visit_ty);
    quote! {
        fn #visit_fn<E>(self, v: #visit_ty) -> Result<Self::Value, E>
        where
            E: #int_enum::__private::serde::de::Error,
        {
            #body
        }
    }
}
