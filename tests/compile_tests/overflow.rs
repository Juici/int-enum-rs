#![deny(overflowing_literals)]

use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum Overflow {
    Max = 255,
    OverMax = 256,
}

fn main() {}
