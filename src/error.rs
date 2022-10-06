use core::fmt;

use crate::IntEnum;

/// Error when attempting to convert an integer into an enum.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct IntEnumError<T: IntEnum> {
    value: T::Int,
}

impl<T: IntEnum> IntEnumError<T> {
    #[doc(hidden)]
    pub fn __new(n: T::Int) -> Self {
        Self { value: n }
    }

    /// Returns the value that could not be converted.
    pub fn value(&self) -> T::Int {
        self.value
    }
}

impl<T: IntEnum> fmt::Display for IntEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown variant: {}", self.value)
    }
}

impl<T: IntEnum> fmt::Debug for IntEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IntEnumError").field("value", &self.value).finish()
    }
}

#[cfg(feature = "std")]
impl<T: IntEnum> std::error::Error for IntEnumError<T> {}
