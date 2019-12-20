extern crate proc_macro;

mod base;
mod serde;

use quote::quote;
use syn::ItemEnum;
use syn::{parse_macro_input, parse_quote, Path};

#[proc_macro_attribute]
pub fn int_enum(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);
    base::add_missing_debug(&mut input.attrs);

    let enum_ = input.ident.clone();
    let int_ = match syn::parse::<base::IntType>(args) {
        Ok(int_) => int_.ty,
        Err(err) => {
            let err = err.to_compile_error();
            return quote!(#input #err).into();
        }
    };

    let crate_ = base::crate_name();
    let core: Path = parse_quote!(#crate_::__core);

    let base_impl = base::base_impl(&input, &crate_, &core, &enum_, &int_);
    let serde_impls = serde::serde_impls(&crate_, &core, &enum_, &int_);

    proc_macro::TokenStream::from(quote! {
        #input
        #base_impl
        #serde_impls
    })
}
