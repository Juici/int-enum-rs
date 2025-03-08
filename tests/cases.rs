use int_enum::IntEnum;

#[test]
fn basic() {
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(u8)]
    enum Basic {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    assert_eq!(u8::from(Basic::Zero), 0);
    assert_eq!(u8::from(Basic::One), 1);
    assert_eq!(u8::from(Basic::Two), 2);

    assert_eq!(Basic::try_from(0), Ok(Basic::Zero));
    assert_eq!(Basic::try_from(1), Ok(Basic::One));
    assert_eq!(Basic::try_from(2), Ok(Basic::Two));
    assert_eq!(Basic::try_from(3), Err(3));
}

#[test]
fn signed() {
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(i8)]
    enum Signed {
        MinusOne = -1,
        One = 1,
    }

    assert_eq!(i8::from(Signed::MinusOne), -1);
    assert_eq!(i8::from(Signed::One), 1);

    assert_eq!(Signed::try_from(-1), Ok(Signed::MinusOne));
    assert_eq!(Signed::try_from(1), Ok(Signed::One));
    assert_eq!(Signed::try_from(0), Err(0));
}

#[test]
fn with_align() {
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(u16, align(4))]
    enum WithAlign {
        A = 0x41,
        B = 0x42,
    }

    assert_eq!(u16::from(WithAlign::A), 0x41);
    assert_eq!(u16::from(WithAlign::B), 0x42);

    assert_eq!(WithAlign::try_from(0x41), Ok(WithAlign::A));
    assert_eq!(WithAlign::try_from(0x42), Ok(WithAlign::B));
    assert_eq!(WithAlign::try_from(0x43), Err(0x43));
}

#[test]
fn discriminant_expr() {
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(u16)]
    enum Expr {
        Up = 1 << 0,
        Down = 1 << 1,
        Left = 1 << 2,
        Right = 1 << 3,
    }

    assert_eq!(u16::from(Expr::Up), 1 << 0);
    assert_eq!(u16::from(Expr::Down), 1 << 1);
    assert_eq!(u16::from(Expr::Left), 1 << 2);
    assert_eq!(u16::from(Expr::Right), 1 << 3);

    assert_eq!(Expr::try_from(1 << 0), Ok(Expr::Up));
    assert_eq!(Expr::try_from(1 << 1), Ok(Expr::Down));
    assert_eq!(Expr::try_from(1 << 2), Ok(Expr::Left));
    assert_eq!(Expr::try_from(1 << 3), Ok(Expr::Right));
    assert_eq!(Expr::try_from(1 << 4), Err(1 << 4));
}

#[test]
fn no_discriminant() {
    // Ensuring conformity with the documented behavior of implicit discriminators
    // https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.implicit
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(i16)]
    enum Signed {
        Zero,
        MinusOne = -1,
        One = 1,
        Two,
        MinusFive = -5,
        MinusFour,
    }

    assert_eq!(i16::from(Signed::Zero), 0);
    assert_eq!(i16::from(Signed::MinusOne), -1);
    assert_eq!(i16::from(Signed::One), 1);
    assert_eq!(i16::from(Signed::Two), 2);
    assert_eq!(i16::from(Signed::MinusFive), -5);
    assert_eq!(i16::from(Signed::MinusFour), -4);

    assert_eq!(Signed::try_from(0), Ok(Signed::Zero));
    assert_eq!(Signed::try_from(-1), Ok(Signed::MinusOne));
    assert_eq!(Signed::try_from(1), Ok(Signed::One));
    assert_eq!(Signed::try_from(2), Ok(Signed::Two));
    assert_eq!(Signed::try_from(-5), Ok(Signed::MinusFive));
    assert_eq!(Signed::try_from(-4), Ok(Signed::MinusFour));
    assert_eq!(Signed::try_from(-3), Err(-3));
    assert_eq!(Signed::try_from(3), Err(3));

    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(u8)]
    enum Unsigned {
        One = 1,
        Two,
        Four = 4,
        Five,
    }

    assert_eq!(u8::from(Unsigned::One), 1);
    assert_eq!(u8::from(Unsigned::Two), 2);
    assert_eq!(u8::from(Unsigned::Four), 4);
    assert_eq!(u8::from(Unsigned::Five), 5);

    assert_eq!(Unsigned::try_from(1), Ok(Unsigned::One));
    assert_eq!(Unsigned::try_from(2), Ok(Unsigned::Two));
    assert_eq!(Unsigned::try_from(4), Ok(Unsigned::Four));
    assert_eq!(Unsigned::try_from(5), Ok(Unsigned::Five));
    assert_eq!(Unsigned::try_from(3), Err(3));
}
