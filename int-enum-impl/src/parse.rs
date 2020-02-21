use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Ident, LitInt, Meta, NestedMeta, Path, Result, Token, Visibility};

pub struct Int {
    pub neg: Option<Token![-]>,
    pub lit: LitInt,
}

impl Parse for Int {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Int {
            neg: if input.peek(Token![-]) {
                Some(input.parse()?)
            } else {
                None
            },
            lit: input.parse()?,
        })
    }
}

impl ToTokens for Int {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(neg) = &self.neg {
            neg.to_tokens(tokens);
        }
        self.lit.to_tokens(tokens);
    }
}

struct IntEnumVariant {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub eq_token: Token![=],
    pub discriminant: Int,
}

impl Parse for IntEnumVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(IntEnumVariant {
            attrs: Attribute::parse_outer(input)?,
            ident: input.parse()?,
            eq_token: input.parse()?,
            discriminant: input.parse()?,
        })
    }
}

impl IntEnumVariant {
    fn into_pair(self, repr: &Ident) -> (Ident, Int) {
        let IntEnumVariant {
            ident,
            mut discriminant,
            ..
        } = self;

        // Create suffixed literal, seems to prevent duplicate error messages.
        let lit = LitInt::new(
            &format!("{}{}", discriminant.lit.base10_digits(), repr),
            Span::call_site(),
        );

        discriminant.lit = lit;

        (ident, discriminant)
    }
}

pub struct IntEnum {
    pub repr: Ident,
    pub ident: Ident,
    pub variants: Vec<(Ident, Int)>,
}

impl Parse for IntEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = Attribute::parse_outer(input)?;

        // `#[repr(...)]` type.
        let repr = repr_from_attrs(&attrs)?;

        let _ = input.parse::<Visibility>()?;
        let _ = input.parse::<Token![enum]>()?;

        // Enum ident.
        let ident = input.parse::<Ident>()?;

        // Generics are not supported.
        if input.peek(Token![<]) {
            return Err(input.error("generics are not supported for IntEnum"));
        }

        // Parse braces from `input`.
        let content: ParseBuffer;
        let _ = syn::braced!(content in input);

        // Parse variants.
        let mut variants = Vec::new();
        loop {
            if content.is_empty() {
                break;
            }
            let variant = content.parse::<IntEnumVariant>()?;
            variants.push(variant.into_pair(&repr));

            if content.is_empty() {
                break;
            }
            let _ = content.parse::<Token![,]>()?;
        }

        Ok(IntEnum {
            repr,
            ident,
            variants,
        })
    }
}

fn repr_from_attrs(attrs: &[Attribute]) -> Result<Ident> {
    for attr in attrs {
        if let Meta::List(meta) = attr.parse_meta()? {
            // Only care about `#[repr(...)]` attribute.
            if !meta.path.is_ident("repr") {
                continue;
            }

            let mut iter = meta.nested.iter();

            // Get the repr.
            let repr = match iter.next() {
                Some(next) => next,
                None => return Err(Error::new(meta.span(), "repr missing int type")),
            };

            // Error if more than one repr.
            {
                let mut errs = None::<Error>;

                // Combine errors.
                while let Some(next) = iter.next() {
                    let err = Error::new(next.span(), "only one repr expected");

                    match &mut errs {
                        Some(errs) => errs.combine(err),
                        errs @ None => *errs = Some(err),
                    }
                }

                if let Some(errs) = errs {
                    return Err(errs);
                }
            }

            let repr = match repr {
                NestedMeta::Meta(Meta::Path(path)) => path,
                repr => return Err(Error::new(repr.span(), "invalid int type")),
            };

            return validate_repr(repr);
        }
    }

    Err(Error::new(Span::call_site(), "no #[repr(...)] found"))
}

fn validate_repr(path: &Path) -> Result<Ident> {
    let ident = match path.get_ident() {
        Some(ident) => ident,
        None => return Err(Error::new(path.span(), "invalid int type")),
    };

    #[rustfmt::skip]
    const VALID_REPRS: [&str; 12] = [
        "u8", "u16", "u32", "u64", "u128", "usize",
        "i8", "i16", "i32", "i64", "i128", "isize",
    ];

    for &repr in &VALID_REPRS {
        if ident == repr {
            return Ok(ident.clone());
        }
    }

    Err(Error::new(ident.span(), "invalid int type"))
}
