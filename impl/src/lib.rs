extern crate proc_macro;

mod ast;
mod expand;
mod serde;
mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntEnum)]
pub fn int_enum_derive(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).unwrap_or_else(|err| err.to_compile_error()).into()
}
