#![cfg_attr(int_enum_test_no_std, no_std)]

use int_enum::{int_enum, IntEnum};

#[int_enum(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Basic {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn test_as_int() {
    assert_eq!(1, Basic::One.to_int());
    assert_eq!(2, Basic::Two.to_int());
    assert_eq!(3, Basic::Three.to_int());
}

#[test]
fn test_from_int() {
    assert_eq!(Basic::One, Basic::from_int(1).unwrap());
    assert_eq!(Basic::Two, Basic::from_int(2).unwrap());
    assert_eq!(Basic::Three, Basic::from_int(3).unwrap());

    assert!(Basic::from_int(4).is_err());
}
