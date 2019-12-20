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
    let int_type = parse_macro_input!(args as IntType);
    let mut input = parse_macro_input!(input as ItemEnum);

    add_missing_debug(&mut input.attrs);

    let ItemEnum {
        ident: enum_type,
        variants,
        ..
    } = &input;

    let crate_name = proc_macro_crate::crate_name("int-enum").expect("int-enum in Cargo.toml");
    let crate_name = Ident::new(&crate_name, Span::call_site());

    let core = quote!(#crate_name::__core);

    let as_int_branches = variants.iter().map(|var| {
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

            fn as_int(&self) -> Self::Int {
                match *self {
                    #(#as_int_branches)*
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

    expanded.into()
}
