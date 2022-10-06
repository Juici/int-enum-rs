#![cfg(not(feature = "serde"))]
#![cfg_attr(int_enum_test_repr128, feature(repr128))]

macro_rules! basic_tests {
    ($($ty:tt)*) => {$(
        pub mod $ty {
            use int_enum::IntEnum;

            #[repr($ty)]
            #[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
            pub enum Basic {
                One = 1,
                Two = 2,
                Three = 3,
            }

            #[test]
            fn test_as_int() {
                assert_eq!(1, Basic::One.int_value());
                assert_eq!(2, Basic::Two.int_value());
                assert_eq!(3, Basic::Three.int_value());
            }

            #[test]
            fn test_from_int() {
                assert_eq!(Basic::One, Basic::from_int(1).unwrap());
                assert_eq!(Basic::Two, Basic::from_int(2).unwrap());
                assert_eq!(Basic::Three, Basic::from_int(3).unwrap());

                assert!(Basic::from_int(4).is_err());
            }
        }
    )*};
}

basic_tests!(u8 u16 u32 u64 usize);
basic_tests!(i8 i16 i32 i64 isize);

#[cfg(int_enum_test_repr128)]
basic_tests!(u128 i128);
