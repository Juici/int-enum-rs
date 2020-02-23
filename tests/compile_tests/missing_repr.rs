use int_enum::IntEnum;

#[derive(Clone, Copy, IntEnum)]
pub enum MissingRepr {
    Zero = 0,
    One = 1,
}

fn main() {}
