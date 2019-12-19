use core::any::type_name;
use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;

use crate::IntEnum;

#[cfg(not(feature = "std"))]
pub trait Error: Debug + Display {}

#[cfg(feature = "std")]
pub use std::error::Error;

/// An error when attempting to convert an integer into an enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntEnumError<Enum: IntEnum> {
    ty: PhantomData<Enum>,
    value: Enum::Int,
}

impl<Enum: IntEnum> IntEnumError<Enum> {
    #[doc(hidden)]
    pub fn __new(n: Enum::Int) -> Self {
        Self {
            ty: PhantomData,
            value: n,
        }
    }

    /// Returns the value that could not be converted.
    pub fn value(&self) -> Enum::Int {
        self.value
    }
}

impl<Enum: IntEnum> Display for IntEnumError<Enum> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid integer value for enum {}: {}",
            type_name::<Enum>(),
            self.value
        )
    }
}

impl<Enum: IntEnum> Error for IntEnumError<Enum> {}
