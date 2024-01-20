#![cfg_attr(proc_macro_span, feature(proc_macro_span))]

extern crate proc_macro;

mod ast;
mod expand;
mod ext;

use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::{parse_macro_input, DeriveInput};

pub(crate) type Result<T, E = Diagnostic> = std::result::Result<T, E>;

/// Derive macro for conversion between integer and enum.
///
/// # Example
///
/// ```
/// use int_enum::IntEnum;
///
/// #[repr(u8)]
/// #[derive(Debug, PartialEq, IntEnum)]
/// pub enum Ascii {
///     UpperA = b'A',
///     UpperB = b'B',
/// }
///
/// assert_eq!(u8::from(Ascii::UpperA), b'A');
/// assert_eq!(u8::from(Ascii::UpperB), b'B');
///
/// assert_eq!(Ascii::try_from(b'A'), Ok(Ascii::UpperA));
/// assert_eq!(Ascii::try_from(b'B'), Ok(Ascii::UpperB));
/// assert_eq!(Ascii::try_from(b'C'), Err(b'C'));
/// ```
#[proc_macro_derive(IntEnum)]
pub fn int_enum_derive(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(input).unwrap_or_else(|d| d.emit_as_item_tokens()).into()
}
