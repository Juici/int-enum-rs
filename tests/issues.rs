use int_enum::IntEnum;

#[test]
fn issue17() {
    #[derive(Debug, PartialEq, IntEnum)]
    #[repr(u8)]
    enum Issue17 {
        Error = 0,
    }
}
