#![deny(overflowing_literals)]

use int_enum::IntEnum;

#[repr(i8)]
#[derive(Clone, Copy, IntEnum)]
pub enum Underflow {
    UnderMin = -129,
    Min = -128,
}

fn main() {}
