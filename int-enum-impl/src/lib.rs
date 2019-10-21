extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn int_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let int_type = parse_macro_input!(args as Ident);
    let input = parse_macro_input!(input as ItemEnum);

    let ItemEnum {
        ident: enum_type,
        variants,
        ..
    } = &input;

    let as_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        let val = match &var.discriminant {
            Some((_, int_value)) => quote!(Some(#int_value)),
            None => quote!(None),
        };

        quote!(#enum_type::#case => #val,)
    });

    let from_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        match &var.discriminant {
            Some((_, int_value)) => quote!(#int_value => Some(#enum_type::#case),),
            None => quote!(),
        }
    });

    let expanded = quote! {
        #input

        impl IntEnum for #enum_type {
            type Int = #int_type;

            fn as_int(&self) -> Option<#int_type> {
                match *self {
                    #(#as_int_branches)*
                    _ => None,
                }
            }

            fn from_int(int: #int_type) -> Option<#enum_type> {
                match int {
                    #(#from_int_branches)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
