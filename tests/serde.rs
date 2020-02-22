#![cfg(feature = "serde")]
#![cfg_attr(int_enum_test_no_std, no_std)]
#![cfg_attr(int_enum_test_repr128, feature(repr128))]

macro_rules! serde_tests {
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
            fn test_serialize() {
                assert_eq!(stringify!($min), serde_json::to_string(&Bounds::Min).unwrap());
                assert_eq!(stringify!($max), serde_json::to_string(&Bounds::Max).unwrap());
            }

            #[test]
            fn test_deserialize() {
                assert_eq!(Bounds::Min, serde_json::from_str(stringify!($min)).unwrap());
                assert_eq!(Bounds::Max, serde_json::from_str(stringify!($max)).unwrap());
            }
        }
    )*};
}

serde_tests! {
    (u8 0 255)
    (u16 0 65535)
    (u32 0 4294967295)
    (u64 0 18446744073709551615)
}

serde_tests! {
    (i8 -128 127)
    (i16 -32768 32767)
    (i32 -2147483648 2147483647)
    (i64 -9223372036854775808 9223372036854775807)
}

#[cfg(int_enum_test_repr128)]
serde_tests! {
    (u128 0 340282366920938463463374607431768211455)
    (i128 -170141183460469231731687303715884105728 170141183460469231731687303715884105727)
}

#[cfg(target_pointer_width = "16")]
serde_tests! {
    (usize 0 65535)
    (isize -32768 32767)
}

#[cfg(target_pointer_width = "32")]
serde_tests! {
    (usize 0 4294967295)
    (isize -2147483648 2147483647)
}

#[cfg(target_pointer_width = "64")]
serde_tests! {
    (usize 0 18446744073709551615)
    (isize -9223372036854775808 9223372036854775807)
}
