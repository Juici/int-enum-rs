#![feature(arbitrary_enum_discriminant)]

use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum Generics<T> {
    Zero = 0,
    One = 1,
    Marker(T),
}

fn main() {}
