use core::fmt;

use proc_macro2::{Ident, TokenStream};
use quote::{IdentFragment, ToTokens};
use syn::{Data, DeriveInput, Error, Expr, Meta, NestedMeta, Result};

pub struct Input<'a> {
    pub original: &'a DeriveInput,
    pub ident: Ident,
    pub repr: Repr,
    pub variants: Vec<(Ident, Expr)>,
}

pub struct Repr {
    pub ident: Ident,
    pub style: IntStyle,
    pub size: IntSize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum IntStyle {
    Signed,
    Unsigned,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum IntSize {
    _8 = 8,
    _16 = 16,
    _32 = 32,
    _64 = 64,
    _128 = 128,
    Size,
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        let data = match &node.data {
            Data::Enum(data) => Ok(data),
            _ => Err(Error::new_spanned(node, "only enums are supported")),
        }?;

        let repr = Repr::from_attrs(node)?;

        let mut variants = Ok(Vec::with_capacity(data.variants.len()));
        for variant in &data.variants {
            match &variant.discriminant {
                Some((_, discr)) => {
                    if let Ok(variants) = &mut variants {
                        variants.push((variant.ident.clone(), discr.clone()));
                    }
                }
                None => {
                    let err = Error::new_spanned(variant, "missing discriminator");
                    match &mut variants {
                        Ok(_) => variants = Err(err),
                        Err(errs) => errs.combine(err),
                    }
                }
            }
        }
        let variants = variants?;

        Ok(Input { original: node, ident: node.ident.clone(), repr, variants })
    }
}

const INVALID_REPR: &str = "invalid repr, expected one of:
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize";

impl Repr {
    fn from_attrs(node: &DeriveInput) -> Result<Self> {
        for attr in &node.attrs {
            if let Meta::List(meta) = attr.parse_meta()? {
                // We only care about `#[repr(...)]` attributes.
                if !meta.path.is_ident("repr") {
                    continue;
                }

                let mut iter = meta.nested.iter();

                let repr = match iter.next() {
                    Some(r) => r,
                    // Ignore empty `#[repr()]`.
                    None => continue,
                };

                if iter.next().is_some() {
                    return Err(Error::new_spanned(&meta.nested, "conflicting reprs"));
                }

                let repr = match repr {
                    NestedMeta::Meta(Meta::Path(path)) => path.get_ident(),
                    _ => None,
                }
                .ok_or_else(|| Error::new_spanned(repr, INVALID_REPR))?;

                return Repr::from_ident(repr);
            }
        }

        Err(Error::new_spanned(node, "missing `repr` attribute"))
    }

    fn from_ident(repr: &Ident) -> Result<Self> {
        fn invalid_repr(repr: &Ident) -> Error {
            Error::new(repr.span(), INVALID_REPR)
        }

        let s = repr.to_string();
        let (signed, size) = match s.as_bytes() {
            [b'i', s @ ..] => (IntStyle::Signed, s),
            [b'u', s @ ..] => (IntStyle::Unsigned, s),
            _ => return Err(invalid_repr(repr)),
        };
        let size = match size {
            b"8" => IntSize::_8,
            b"16" => IntSize::_16,
            b"32" => IntSize::_32,
            b"64" => IntSize::_64,
            b"128" => IntSize::_128,
            b"size" => IntSize::Size,
            _ => return Err(invalid_repr(repr)),
        };

        Ok(Repr { ident: repr.clone(), style: signed, size })
    }
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.ident, f)
    }
}

impl ToTokens for Repr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
    }
}

impl IdentFragment for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        IdentFragment::fmt(&self.ident, f)
    }
}

impl IdentFragment for IntStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntStyle::Signed => write!(f, "i"),
            IntStyle::Unsigned => write!(f, "u"),
        }
    }
}

impl IdentFragment for IntSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntSize::_8 => write!(f, "8"),
            IntSize::_16 => write!(f, "16"),
            IntSize::_32 => write!(f, "32"),
            IntSize::_64 => write!(f, "64"),
            IntSize::_128 => write!(f, "128"),
            IntSize::Size => write!(f, "size"),
        }
    }
}
