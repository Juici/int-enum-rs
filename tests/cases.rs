use int_enum::IntEnum;

#[test]
fn basic() {
    #[repr(u8)]
    #[derive(Debug, PartialEq, IntEnum)]
    pub enum Basic {
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
    #[repr(i8)]
    #[derive(Debug, PartialEq, IntEnum)]
    pub enum Signed {
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
    #[repr(u16, align(4))]
    #[derive(Debug, PartialEq, IntEnum)]
    pub enum WithAlign {
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
    #[repr(u16)]
    #[derive(Debug, PartialEq, IntEnum)]
    pub enum Expr {
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
