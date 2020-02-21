use core::cmp::{Eq, Ord};
use core::fmt::{Debug, Display};
use core::marker::{Copy, Sized};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

/// A primitive integer type.
pub trait PrimInt:
    Sized
    + Copy
    + Eq
    + Ord
    + Debug
    + Display
    + Add
    + Sub
    + Mul
    + Div
    + Rem
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + private::Sealed
{
}

mod private {
    pub trait Sealed {}
}

macro_rules! impl_prim_int {
    ($($T:ty)*) => {$(
        impl private::Sealed for $T {}
        impl PrimInt for $T {}
    )*};
}

impl_prim_int!(i8 i16 i32 i64 i128 isize);
impl_prim_int!(u8 u16 u32 u64 u128 usize);
