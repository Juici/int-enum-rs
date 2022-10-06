use proc_macro_crate::FoundCrate;
use quote::format_ident;
use syn::{parse_quote, Path};

pub fn with_crate_path<F: Fn(&Path) -> R, R>(f: F) -> R {
    let int_enum = match proc_macro_crate::crate_name("int-enum") {
        Ok(FoundCrate::Name(name)) => format_ident!("{}", name),
        Ok(FoundCrate::Itself) | Err(_) => format_ident!("int_enum"),
    };
    let path = parse_quote!(::#int_enum);
    f(&path)
}
