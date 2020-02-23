use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, IntEnum)]
pub enum MissingDiscriminant {
    Zero = 0,
    NoDiscriminant,
}

fn main() {}
