use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Enum {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn from_enum() {
    assert_eq!(u8::from(Enum::One), 1);
    assert_eq!(u8::from(Enum::Two), 2);
    assert_eq!(u8::from(Enum::Three), 3);
}

#[test]
fn try_from_int() {
    assert_eq!(Enum::try_from(1), Ok(Enum::One));
    assert_eq!(Enum::try_from(2), Ok(Enum::Two));
    assert_eq!(Enum::try_from(3), Ok(Enum::Three));

    assert_eq!(Enum::try_from(4), Err(4));
}
