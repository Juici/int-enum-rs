//! A procedural macro for conversion between integer and enum types.

#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod int;

use core::marker::{Copy, Sized};
use core::result::Result;

use self::int::PrimInt;

/// Trait used for implementations of integer and enum conversions.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use std::error::Error;
/// use int_enum::IntEnum;
///
/// #[repr(u8)]
/// #[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
/// pub enum SmallInt {
///     One = 1,
///     Two = 2,
/// }
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # #[cfg(feature = "std")]
/// # {
/// assert_eq!(1, SmallInt::One.int_value());
/// assert_eq!(2, SmallInt::Two.int_value());
///
/// assert_eq!(SmallInt::One, SmallInt::from_int(1)?);
/// assert_eq!(SmallInt::Two, SmallInt::from_int(2)?);
///
/// assert!(SmallInt::from_int(5).is_err());
/// # }
/// # Ok(())
/// # }
/// ```
///
/// Serde support (requires feature `serialize`):
///
/// ```
/// # use std::error::Error;
/// use int_enum::IntEnum;
///
/// #[repr(i8)]
/// #[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
/// enum Num {
///     MinusThree = -3,
///     Zero = 0,
///     Five = 5,
/// }
///
/// # #[cfg(feature = "serde")]
/// # fn main() -> Result<(), Box<dyn Error>> {
/// assert_eq!("-3", serde_json::to_string(&Num::MinusThree)?);
/// assert_eq!("0", serde_json::to_string(&Num::Zero)?);
/// assert_eq!("5", serde_json::to_string(&Num::Five)?);
///
/// assert_eq!(Num::MinusThree, serde_json::from_str("-3")?);
/// assert_eq!(Num::Zero, serde_json::from_str("0")?);
/// assert_eq!(Num::Five, serde_json::from_str("5")?);
///
/// assert!(serde_json::from_str::<Num>("4").is_err());
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "serde"))]
/// # fn main() {}
/// ```
///
/// `From` and `TryFrom` support (requires `convert` feature):
///
/// ```
/// # use std::error::Error;
/// use std::convert::TryFrom;
/// use int_enum::IntEnum;
///
/// #[repr(u16)]
/// #[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
/// enum Value {
///     A = 1_000,
///     B = 1_001,
///     C = 1_002,
/// }
///
/// # #[cfg(feature = "convert")]
/// # fn main() -> Result<(), Box<dyn Error>> {
/// assert_eq!(1_000, u16::from(Value::A));
/// assert_eq!(1_001, u16::from(Value::B));
/// assert_eq!(1_002, u16::from(Value::C));
///
/// assert_eq!(Value::A, Value::try_from(1_000)?);
/// assert_eq!(Value::B, Value::try_from(1_001)?);
/// assert_eq!(Value::C, Value::try_from(1_002)?);
///
/// assert!(Value::try_from(2_000).is_err());
/// # Ok(())
/// # }
/// # #[cfg(not(feature = "convert"))]
/// # fn main() {}
/// ```
pub trait IntEnum: Copy {
    /// Primitive integer type for conversions.
    type Int: PrimInt;

    /// Returns the integer value of the enum.
    fn int_value(self) -> Self::Int;

    /// Attempts to convert an integer into the enum.
    ///
    /// # Errors
    ///
    /// If `n` is not a variant in the enum.
    fn from_int(n: Self::Int) -> Result<Self, IntEnumError<Self>>
    where
        Self: Sized;
}

#[doc(inline)]
pub use self::error::IntEnumError;

#[doc(hidden)]
pub use int_enum_impl::*;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use core::fmt;
    pub use core::format_args;

    pub use core::convert::{From, TryFrom};
    pub use core::result::Result;

    #[cfg(feature = "serde")]
    pub use serde;
}
