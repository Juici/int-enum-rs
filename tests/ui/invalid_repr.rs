use int_enum::IntEnum;

#[derive(Clone, Copy, IntEnum)]
#[repr(align(1))]
pub enum InvalidRepr {
    Zero = 0,
    One = 1,
}

fn main() {}
