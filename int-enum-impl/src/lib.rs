extern crate proc_macro;

mod base;
mod convert;
mod dummy;
mod parse;
mod serde;

use quote::quote;
use syn::parse_macro_input;

use crate::parse::IntEnum;

#[proc_macro_derive(IntEnum)]
pub fn int_enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as IntEnum);

    let int_enum_impl = base::int_enum_impl(&input);
    let convert_impl = convert::convert_impl(&input);
    let serde_impl = serde::serde_impl(&input);

    proc_macro::TokenStream::from(quote! {
        #int_enum_impl
        #convert_impl
        #serde_impl
    })
}
