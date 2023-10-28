#![cfg_attr(proc_macro_span, feature(proc_macro_span))]

extern crate proc_macro;

mod ast;
mod expand;
mod ext;

use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::{parse_macro_input, DeriveInput};

pub(crate) type Result<T, E = Diagnostic> = std::result::Result<T, E>;

#[proc_macro_derive(IntEnum)]
pub fn int_enum_derive(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(input).unwrap_or_else(|d| d.emit_as_item_tokens()).into()
}
