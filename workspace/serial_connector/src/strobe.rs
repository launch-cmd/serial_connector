#[repr(u8)]
#[derive(PartialEq)]
pub enum Strobe {
    Address = 128,
    Data = 64,
}

impl Strobe {
    pub fn as_u8(&self) -> u8 {
        match self {
            Strobe::Address => 128_u8,
            Strobe::Data => 64_u8,
        }
    }
}
