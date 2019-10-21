use int_enum::*;

#[int_enum(u64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn basic_as_int() {
    assert_eq!(Basic::One.as_int(), Some(1));
    assert_eq!(Basic::Two.as_int(), Some(2));
    assert_eq!(Basic::Three.as_int(), Some(3));
}

#[test]
fn basic_from_int() {
    assert_eq!(Basic::from_int(1), Some(Basic::One));
    assert_eq!(Basic::from_int(2), Some(Basic::Two));
    assert_eq!(Basic::from_int(3), Some(Basic::Three));

    assert_eq!(Basic::from_int(4), None);
}
