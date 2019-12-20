use int_enum::*;

#[int_enum(u8)]
enum InvalidVariant {
    One = 1,
    Two = 2,
    Invalid,
    AlsoInvalid,
}

fn main() {}
