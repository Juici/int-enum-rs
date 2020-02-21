use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

// Adapted from `wrap_in_const` function in `serde_derive`.
// https://github.com/serde-rs/serde/blob/master/serde_derive/src/dummy.rs#L6
pub fn wrap_in_const(trait_: &str, ty: &Ident, code: TokenStream) -> TokenStream {
    let ty = ty.to_string();
    let unraw_ty = ty.trim_start_matches("r#");

    let dummy_const = Ident::new(
        &format!("_IMPL_{}_FOR_{}", trait_, unraw_ty),
        Span::call_site(),
    );

    let use_int_enum = {
        let crate_name =
            proc_macro_crate::crate_name("int-enum").expect("missing int-enum in `Cargo.toml`");
        let crate_path = Ident::new(&crate_name, Span::call_site());

        quote! {
            #[allow(unknown_lints)]
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            #[allow(rust_2018_idioms)]
            extern crate #crate_path as _int_enum;
        }
    };

    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            #use_int_enum
            #code
        };
    }
}
