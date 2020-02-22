//! A procedural macro for conversion between integer and enum types.

#![doc(html_root_url = "https://docs.rs/int-enum/*")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

mod error;
mod int;

use core::marker::{Copy, Sized};
use core::result::Result;

use self::int::PrimInt;

/// Trait used for implementations of integer and enum conversions.
pub trait IntEnum: Copy {
    /// Primitive integer type for conversions.
    type Int: PrimInt;

    /// Returns the integer value of the enum.
    fn int_value(self) -> Self::Int;

    /// Attempts to convert an integer into the enum.
    fn from_int(n: Self::Int) -> Result<Self, IntEnumError<Self>>
    where
        Self: Sized;
}

#[doc(inline)]
pub use self::error::IntEnumError;

#[doc(hidden)]
pub use int_enum_impl::IntEnum;

#[doc(hidden)]
pub mod export {
    pub use core::fmt;
    pub use core::result::Result;

    #[cfg(feature = "convert")]
    pub use core::convert::{From, TryFrom};

    #[cfg(feature = "serde")]
    pub use serde_crate as serde;
}
