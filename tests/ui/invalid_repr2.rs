use int_enum::IntEnum;

#[repr(u8, align(1))]
#[derive(Clone, Copy, IntEnum)]
pub enum InvalidRepr {
    Zero = 0,
    One = 1,
}

fn main() {}
