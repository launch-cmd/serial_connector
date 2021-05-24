#[repr(u8)]
#[derive(PartialEq)]
pub enum CardNo {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Fifteen = 15,
}

impl CardNo {
    pub fn as_u8(&self) -> u8 {
        match self {
            CardNo::One => 1_u8,
            CardNo::Two => 2_u8,
            CardNo::Three => 3_u8,
            CardNo::Four => 4_u8,
            CardNo::Five => 5_u8,
            CardNo::Six => 6_u8,
            CardNo::Seven => 7_u8,
            CardNo::Fifteen => 15_u8,
        }
    }
}
