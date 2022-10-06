#![allow(clippy::enum_clike_unportable_variant, clippy::unreadable_literal)]
#![cfg(not(feature = "serde"))]
#![cfg_attr(int_enum_test_repr128, feature(repr128))]

macro_rules! bounds_tests {
    ($( ($ty:tt $min:literal $max:literal) )*) => {$(
        pub mod $ty {
            use int_enum::IntEnum;

            #[repr($ty)]
            #[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
            pub enum Bounds {
                Min = $min,
                Max = $max,
            }

            #[test]
            fn test_as_int() {
                assert_eq!($ty::MIN, Bounds::Min.int_value());
                assert_eq!($ty::MAX, Bounds::Max.int_value());
            }

            #[test]
            fn test_from_int() {
                assert_eq!(Bounds::Min, Bounds::from_int($ty::MIN).unwrap());
                assert_eq!(Bounds::Max, Bounds::from_int($ty::MAX).unwrap());
            }
        }
    )*};
}

bounds_tests! {
    (u8 0 255)
    (u16 0 65535)
    (u32 0 4294967295)
    (u64 0 18446744073709551615)
}

bounds_tests! {
    (i8 -128 127)
    (i16 -32768 32767)
    (i32 -2147483648 2147483647)
    (i64 -9223372036854775808 9223372036854775807)
}

#[cfg(int_enum_test_repr128)]
bounds_tests! {
    (u128 0 340282366920938463463374607431768211455)
    (i128 -170141183460469231731687303715884105728 170141183460469231731687303715884105727)
}

#[cfg(target_pointer_width = "16")]
bounds_tests! {
    (usize 0 65535)
    (isize -32768 32767)
}

#[cfg(target_pointer_width = "32")]
bounds_tests! {
    (usize 0 4294967295)
    (isize -2147483648 2147483647)
}

#[cfg(target_pointer_width = "64")]
bounds_tests! {
    (usize 0 18446744073709551615)
    (isize -9223372036854775808 9223372036854775807)
}
