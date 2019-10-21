//! A procedural macro for conversion between integer and enum types.

#![doc(html_root_url = "https://docs.rs/int-enum/*")]
#![deny(missing_docs)]

// Use num-traits PrimInt to assert that the type passed to the macro is a primitive integer type.
use num_traits::PrimInt;

/// Attribute macro taking the integer type for the enum.
///
/// # Example
///
/// ```
/// use int_enum::*;
///
/// #[int_enum(u8)]
/// #[derive(Debug, PartialEq)]
/// pub enum SmallInt {
///     One = 1,
///     Two = 2,
/// }
///
/// fn main() {
///     assert_eq!(SmallInt::One.as_int(), Some(1));
///
///     assert_eq!(SmallInt::from_int(2), Some(SmallInt::Two));
///     assert_eq!(SmallInt::from_int(5), None);
/// }
/// ```
pub use int_enum_impl::int_enum;

/// Trait used for implementations of integer and enum conversions.
pub trait IntEnum {
    /// The primitive integer type.
    type Int: PrimInt;

    /// Gets the integer value of the enum.
    fn as_int(&self) -> Option<Self::Int>;

    /// Gets the enum type for the given integer.
    fn from_int(int: Self::Int) -> Option<Self>
    where
        Self: Sized;
}
