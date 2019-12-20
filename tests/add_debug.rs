#![no_std]

use core::fmt::Debug;

use int_enum::int_enum;

#[int_enum(u8)]
pub enum NoDebug {
    One = 1,
    Two = 2,
}

#[int_enum(u8)]
#[derive(Debug)]
pub enum HasDebug {
    One = 1,
    Two = 2,
}

fn assert_debug<T: Debug>() {}

#[test]
fn test_added_debug() {
    assert_debug::<NoDebug>();
}

#[test]
fn test_has_debug() {
    assert_debug::<HasDebug>();
}
