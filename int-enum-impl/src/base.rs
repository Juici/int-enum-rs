use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, ItemImpl};
use syn::{
    Attribute, Error, Ident, ItemEnum, ItemFn, Meta, NestedMeta, Path, Result, Token, Variant,
};

pub struct IntType {
    pub ty: Ident,
}

impl Parse for IntType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty: Ident = input.parse()?;

        const VALID_TYPES: &[&str] = &[
            "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
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

pub fn add_missing_debug(attrs: &mut Vec<Attribute>) {
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

pub fn crate_name() -> Ident {
    let crate_ = proc_macro_crate::crate_name("int-enum").expect("int-enum in Cargo.toml");
    Ident::new(&crate_, Span::call_site())
}

pub fn base_impl(
    input: &ItemEnum,
    crate_: &Ident,
    core: &Path,
    enum_: &Ident,
    int_: &Ident,
) -> ItemImpl {
    let to_int_fn = to_int_fn(&core, &enum_, &input.variants);
    let from_int_fn = from_int_fn(&crate_, &core, &enum_, &input.variants);

    parse_quote! {
        impl #crate_::IntEnum for #enum_ {
            type Int = #int_;

            #to_int_fn
            #from_int_fn
        }
    }
}

fn to_int_fn(core: &Path, enum_: &Ident, variants: &Punctuated<Variant, Token![,]>) -> ItemFn {
    let to_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        let val = match &var.discriminant {
            Some((_, v)) => quote!(#v),
            None => Error::new_spanned(var, "#[int_enum] not supported for non valued variants")
                .to_compile_error(),
        };

        quote!(#enum_::#case => { #val })
    });

    parse_quote! {
        fn to_int(&self) -> Self::Int {
            match *self {
                #(#to_int_branches)*
                _ => { #core::unreachable!() }
            }
        }
    }
}

fn from_int_fn(
    crate_: &Ident,
    core: &Path,
    enum_: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
) -> ItemFn {
    let from_int_branches = variants.iter().map(|var| {
        let case = &var.ident;
        match &var.discriminant {
            Some((_, int_value)) => {
                quote!(#int_value => { #core::result::Result::Ok(#enum_::#case) })
            }
            None => quote!(),
        }
    });

    parse_quote! {
        fn from_int(n: Self::Int) -> #core::result::Result<Self, #crate_::IntEnumError<Self>> {
            match n {
                #(#from_int_branches)*
                _ => { #core::result::Result::Err(#crate_::IntEnumError::__new(n)) }
            }
        }
    }
}
