use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum NegativeUnsigned {
    MinusOne = -1,
    Zero = 0,
}

fn main() {}
