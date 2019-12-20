#![cfg_attr(int_enum_test_no_std, no_std)]

use int_enum::{int_enum, IntEnum};

/// Docs for the enum.
#[int_enum(i8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Advanced {
    /// Docs for zero.
    Zero = 0,
    /// Docs for twenty.
    Twenty = 20,
    /// Docs for negative five.
    NegativeFive = -5,
}

#[test]
fn test_as_int() {
    assert_eq!(0, Advanced::Zero.to_int());
    assert_eq!(20, Advanced::Twenty.to_int());
    assert_eq!(-5, Advanced::NegativeFive.to_int());
}

#[test]
fn test_from_int() {
    assert_eq!(Advanced::Zero, Advanced::from_int(0).unwrap());
    assert_eq!(Advanced::Twenty, Advanced::from_int(20).unwrap());
    assert_eq!(Advanced::NegativeFive, Advanced::from_int(-5).unwrap());

    assert!(Advanced::from_int(-127i8).is_err());
}
