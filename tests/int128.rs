#![cfg_attr(int_enum_test_no_std, no_std)]

use int_enum::{int_enum, IntEnum};

#[int_enum(u128)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum U128 {
    One = 1,
}

#[int_enum(i128)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum I128 {
    Negative = -31231223124151,
}

#[test]
fn test_as_int() {
    assert_eq!(U128::One.to_int(), 1);
    assert_eq!(I128::Negative.to_int(), -31231223124151);
}

#[test]
fn test_from_int() {
    assert_eq!(U128::One, U128::from_int(1).unwrap());
    assert_eq!(I128::Negative, I128::from_int(-31231223124151).unwrap());
}
