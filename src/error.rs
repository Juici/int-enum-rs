use cfg_if::cfg_if;

use core::any::type_name;
use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;

use crate::IntEnum;

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::error::Error;
    } else {
        /// Error trait similar to the one found in the standard library.
        pub trait Error: Debug + Display {}
    }
}

/// An error when attempting to convert an integer into an enum.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEnumError<T: IntEnum> {
    ty: PhantomData<T>,
    value: T::Int,
}

impl<T: IntEnum> IntEnumError<T> {
    #[doc(hidden)]
    pub fn __new(n: T::Int) -> Self {
        Self {
            ty: PhantomData,
            value: n,
        }
    }

    /// Returns the value that could not be converted.
    pub fn value(&self) -> T::Int {
        self.value
    }
}

impl<T: IntEnum> Display for IntEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid integer value for enum {}: {}",
            type_name::<T>(),
            self.value
        )
    }
}

impl<T: IntEnum> Debug for IntEnumError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("IntEnumError");
        ds.field("ty", &self.ty);
        ds.field("value", &self.value);
        ds.finish()
    }
}

impl<T: IntEnum> Error for IntEnumError<T> {}
