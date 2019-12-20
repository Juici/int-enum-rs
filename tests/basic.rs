use int_enum::*;

#[int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn test_as_int() {
    assert_eq!(Basic::One.as_int(), 1);
    assert_eq!(Basic::Two.as_int(), 2);
    assert_eq!(Basic::Three.as_int(), 3);
}

#[test]
fn test_from_int() {
    assert_eq!(Basic::from_int(1), Ok(Basic::One));
    assert_eq!(Basic::from_int(2), Ok(Basic::Two));
    assert_eq!(Basic::from_int(3), Ok(Basic::Three));

    assert!(Basic::from_int(4).is_err());
}
