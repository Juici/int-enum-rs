extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, parse_quote, Attribute, Error, Ident, ItemEnum, Meta, NestedMeta, Result,
};

struct IntType {
    ty: Ident,
}

impl Parse for IntType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty: Ident = input.parse()?;

        const VALID_TYPES: &[&str] = &[
            "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
        ];

        let valid = VALID_TYPES.iter().any(|t| ty == t);
        if !valid {
            return Err(Error::new_spanned(
                ty,
                "#[int_enum] not supported for this type",
            ));
        }

        Ok(IntType { ty })
    }
}

impl ToTokens for IntType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ty.to_tokens(tokens);
    }
}

fn add_missing_debug(attrs: &mut Vec<Attribute>) {
    let mut missing = true;
    for attr in &attrs[..] {
        if let Ok(meta) = attr.parse_meta() {
            if !meta.path().is_ident("derive") {
                continue;
            }

            if let Meta::List(types) = meta {
                let found = types.nested.iter().any(|m| match m {
                    NestedMeta::Meta(Meta::Path(p)) => p.is_ident("Debug"),
                    _ => false,
                });
                if found {
                    missing = false;
                    break;
                }
            }
        }
    }

    if missing {
        let attr: Attribute = parse_quote!(#[derive(Debug)]);
        attrs.push(attr);
    }
}

#[proc_macro_attribute]
pub fn int_enum(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);
    add_missing_debug(&mut input.attrs);

    let int_type = match syn::parse::<IntType>(args) {
        Ok(int_type) => int_type.ty,
        Err(err) => {
            let err = err.to_compile_error();
            return quote!(#input #err).into();
        }
    };

    let ItemEnum {
        ident: enum_type,
        variants,
        ..
    } = &input;

    let crate_name = proc_macro_crate::crate_name("int-enum").expect("int-enum in Cargo.toml");
    let crate_name = Ident::new(&crate_name, Span::call_site());

    let core = quote!(#crate_name::__core);

    let to_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        let val = match &var.discriminant {
            Some((_, int_value)) => quote!(#int_value),
            None => Error::new_spanned(var, "#[int_enum] not supported for non valued variants")
                .to_compile_error(),
        };

        quote!(#enum_type::#case => { #val })
    });

    let from_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        match &var.discriminant {
            Some((_, int_value)) => {
                quote!(#int_value => { #core::result::Result::Ok(#enum_type::#case) })
            }
            None => quote!(),
        }
    });

    let expanded = quote! {
        #input

        impl #crate_name::IntEnum for #enum_type {
            type Int = #int_type;

            fn to_int(&self) -> Self::Int {
                match *self {
                    #(#to_int_branches)*
                    _ => { #core::unreachable!() }
                }
            }

            fn from_int(n: Self::Int) -> #core::result::Result<Self, #crate_name::IntEnumError<Self>> {
                match n {
                    #(#from_int_branches)*
                    _ => { #core::result::Result::Err(#crate_name::IntEnumError::__new(n)) }
                }
            }
        }
    };

    #[cfg(feature = "serialize")]
    let expanded = {
        let serde = quote!(#crate_name::__serde);

        let int_type_str = int_type.to_string();
        let enum_type_str = enum_type.to_string();

        let ser_fn = Ident::new(&format!("serialize_{}", &int_type_str), Span::call_site());

        let expecting = format!("{} integer", &int_type_str);

        let unknown_value = {
            cfg_if::cfg_if! {
                if #[cfg(any(feature = "std", feature = "alloc"))] {
                    let unknown_value = format!("unknown {} value: {{}}", &enum_type_str);
                    quote!(custom(#crate_name::__format!(#unknown_value, v)))
                } else {
                    let unknown_value = format!("unknown {} value", &enum_type_str);
                    quote!(custom(#unknown_value))
                }
            }
        };

        let visit_fn = match &int_type_str[..] {
            "i8" | "i16" | "i32" | "i64" => quote! {
                fn visit_i64<E>(self, v: i64) -> #core::result::Result<Self::Value, E>
                where
                    E: #serde::de::Error,
                {
                    const MIN: i64 = #core::#int_type::MIN as i64;
                    const MAX: i64 = #core::#int_type::MAX as i64;

                    if MIN <= v && v <= MAX {
                        let v: #core::result::Result<Self::Value, _> = #crate_name::IntEnum::from_int(v as #int_type);
                        if let #core::result::Result::Ok(v) = v {
                            return #core::result::Result::Ok(v);
                        }
                    }

                    #core::result::Result::Err(E::#unknown_value)
                }
            },
            "u8" | "u16" | "u32" | "u64" => quote! {
                fn visit_u64<E>(self, v: u64) -> #core::result::Result<Self::Value, E>
                where
                    E: #serde::de::Error,
                {
                    const MAX: u64 = #core::#int_type::MAX as u64;

                    if v <= MAX {
                        let v: #core::result::Result<Self::Value, _> = #crate_name::IntEnum::from_int(v as #int_type);
                        if let #core::result::Result::Ok(v) = v {
                            return #core::result::Result::Ok(v);
                        }
                    }

                    #core::result::Result::Err(E::#unknown_value)
                }
            },
            "i128" | "u128" => {
                let visit = Ident::new(&format!("visit_{}", &int_type_str), Span::call_site());
                quote! {
                    fn #visit<E>(self, v: #int_type) -> #core::result::Result<Self::Value, E>
                    where
                        E: #serde::de::Error,
                    {
                        #crate_name::IntEnum::from_int(v as #int_type).map_err(|_| E::#unknown_value)
                    }
                }
            }
            _ => unreachable!(),
        };

        quote! {
            #expanded

            impl #serde::Serialize for #enum_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: #serde::Serializer
                {
                    let n: #int_type = #crate_name::IntEnum::to_int(self);
                    serializer.#ser_fn(n)
                }
            }

            impl<'de> #serde::Deserialize<'de> for #enum_type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>
                {
                    struct Visitor;

                    impl<'de> #serde::de::Visitor<'de> for Visitor {
                        type Value = #enum_type;

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
    };

    expanded.into()
}
