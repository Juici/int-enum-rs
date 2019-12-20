#![cfg(feature = "serialize")]
#![cfg_attr(int_enum_test_no_std, no_std)]

use int_enum::int_enum;

#[int_enum(u8)]
#[derive(Eq, PartialEq)]
enum Num {
    One = 1,
    Two = 2,
    Three = 3,
}

#[int_enum(i16)]
#[derive(Eq, PartialEq)]
enum NegNum {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn test_basic_serialize() {
    assert_eq!("1", serde_json::to_string(&Num::One).unwrap());
    assert_eq!("2", serde_json::to_string(&Num::Two).unwrap());
    assert_eq!("3", serde_json::to_string(&Num::Three).unwrap());
}

#[test]
fn test_basic_deserialize() {
    assert_eq!(Num::One, serde_json::from_str("1").unwrap());
    assert_eq!(Num::Two, serde_json::from_str("2").unwrap());
    assert_eq!(Num::Three, serde_json::from_str("3").unwrap());

    assert!(serde_json::from_str::<Num>("4").is_err());
}
