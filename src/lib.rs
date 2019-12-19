//! A procedural macro for conversion between integer and enum types.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/int-enum/*")]
#![deny(missing_docs)]

#[doc(hidden)]
pub extern crate core as __core;

mod error;
mod int;

use core::fmt::Debug;

use self::int::PrimInt;

/// Trait used for implementations of integer and enum conversions.
pub trait IntEnum: Sized + Debug {
    /// Primitive integer type for conversions.
    type Int: PrimInt;

    /// Returns the integer value of the enum.
    fn as_int(&self) -> Self::Int;

    /// Converts an integer into the enum.
    fn from_int(n: Self::Int) -> Result<Self, IntEnumError<Self>>;
}

/// Attribute macro taking the integer type for the enum.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use int_enum::*;
///
/// #[int_enum(u8)]
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum SmallInt {
///     One = 1,
///     Two = 2,
/// }
///
/// assert_eq!(SmallInt::One.as_int(), 1);
///
/// assert_eq!(SmallInt::from_int(2), Ok(SmallInt::Two));
/// assert!(SmallInt::from_int(5).is_err());
/// ```
pub use int_enum_impl::int_enum;

#[doc(inline)]
pub use self::error::IntEnumError;
