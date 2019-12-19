use core::fmt::{Debug, Display};

/// A primitive integer type.
pub trait PrimInt: Sized + Copy + Debug + Display + private::Sealed {}

mod private {
    pub trait Sealed {}
}

macro_rules! impl_prim_int {
    ($($T:ty)*) => {$(
        impl private::Sealed for $T {}
        impl PrimInt for $T {}
    )*};
}

impl_prim_int!(i8 i16 i32 i64 i128);
impl_prim_int!(u8 u16 u32 u64 u128);
