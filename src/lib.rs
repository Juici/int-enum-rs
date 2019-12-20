//! A procedural macro for conversion between integer and enum types.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(html_root_url = "https://docs.rs/int-enum/*")]
#![deny(missing_docs)]

use cfg_if::cfg_if;

// Always re-export this.
#[doc(hidden)]
pub extern crate core as __core;

#[cfg(feature = "std")]
extern crate std;

// Re-export format! macro.
cfg_if! {
    if #[cfg(feature = "std")] {
        #[doc(hidden)]
        pub use std::format as __format;
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;

        #[doc(hidden)]
        pub use alloc::format as __format;
    }
}

// Re-export serde.
cfg_if! {
    if #[cfg(feature = "serialize")] {
        #[doc(hidden)]
        pub use serde as __serde;
    }
}

mod error;
mod int;

use core::fmt::Debug;

use self::int::PrimInt;

/// Trait used for implementations of integer and enum conversions.
pub trait IntEnum: Sized + Debug {
    /// Primitive integer type for conversions.
    type Int: PrimInt;

    /// Returns the integer value of the enum.
    fn to_int(&self) -> Self::Int;

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
/// use int_enum::{int_enum, IntEnum};
///
/// #[int_enum(u8)]
/// #[derive(PartialEq, Eq)]
/// pub enum SmallInt {
///     One = 1,
///     Two = 2,
/// }
///
/// assert_eq!(SmallInt::One.to_int(), 1);
///
/// assert_eq!(SmallInt::from_int(2), Ok(SmallInt::Two));
/// assert!(SmallInt::from_int(5).is_err());
/// ```
///
/// Serde support (requires feature `serialize`):
///
/// ```
/// # use std::error::Error;
/// use int_enum::{int_enum, IntEnum};
///
/// #[int_enum(u8)]
/// enum Num {
///     One = 1,
///     Two = 2,
///     Three = 3,
/// }
///
/// # #[cfg(feature = "serialize")]
/// # fn main() -> Result<(), Box<dyn Error>> {
/// assert_eq!("1", serde_json::to_string(&Num::One)?);
/// assert_eq!("2", serde_json::to_string(&Num::Two)?);
/// assert_eq!("3", serde_json::to_string(&Num::Three)?);
///
/// assert!(serde_json::from_str::<Num>("4").is_err());
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "serialize"))]
/// # fn main() {}
/// ```
pub use int_enum_impl::int_enum;

#[doc(inline)]
pub use self::error::IntEnumError;
