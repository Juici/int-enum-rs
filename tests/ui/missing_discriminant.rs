use int_enum::IntEnum;

#[derive(Clone, Copy, IntEnum)]
#[repr(u8)]
pub enum MissingDiscriminant {
    Zero = 0,
    NoDiscriminant,
}

fn main() {}
