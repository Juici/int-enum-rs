use int_enum::*;

#[int_enum(u8)]
enum IncompatibleType {
    NegativeOne = -1,
    Two = 2,
}

fn main() {}
