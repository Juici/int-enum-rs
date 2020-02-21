#![cfg(feature = "convert")]
#![cfg_attr(int_enum_test_no_std, no_std)]

use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Basic {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn test_as_int() {
    assert_eq!(1, u8::from(Basic::One));
    assert_eq!(2, u8::from(Basic::Two));
    assert_eq!(3, u8::from(Basic::Three));
}

#[test]
fn test_from_int() {
    assert_eq!(Basic::One, Basic::try_from(1).unwrap());
    assert_eq!(Basic::Two, Basic::try_from(2).unwrap());
    assert_eq!(Basic::Three, Basic::try_from(3).unwrap());

    assert!(Basic::try_from(4).is_err());
}
