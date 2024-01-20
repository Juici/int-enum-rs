# derive(IntEnum)

[<img alt="github" src="https://img.shields.io/badge/github-juici/int--enum--rs-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/Juici/int-enum-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/int-enum?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/int-enum)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-int--enum-4d76ae?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/int-enum)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Juici/int-enum-rs/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/Juici/int-enum-rs/actions?query=branch%3Amaster)

This crate provides a convenient derive macro for the core library's [`From`]
and [`TryFrom`] traits for converting between integer and enum types.

[`From`]: https://doc.rust-lang.org/core/convert/trait.From.html
[`TryFrom`]: https://doc.rust-lang.org/core/convert/trait.TryFrom.html

```toml
[dependencies]
int-enum = "1.0"
```

_Compiler support: requires rustc 1.64+_

## Example

```rs
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, IntEnum)]
pub enum Ascii {
   UpperA = b'A',
   UpperB = b'B',
}

assert_eq!(u8::from(Ascii::UpperA), b'A');
assert_eq!(u8::from(Ascii::UpperB), b'B');

assert_eq!(Ascii::try_from(b'A'), Ok(Ascii::UpperA));
assert_eq!(Ascii::try_from(b'B'), Ok(Ascii::UpperB));
assert_eq!(Ascii::try_from(b'C'), Err(b'C'));
```

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT License](LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
