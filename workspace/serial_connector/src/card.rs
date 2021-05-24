use crate::CardNo;
use crate::Strobe;

pub struct Card {
    pub number: CardNo,
    pub speed: u8,
    pub block_info: [u8; 1],
}

impl Card {
    pub fn select(&self, port: &mut dyn serialport::SerialPort) -> Result<(), std::io::Error> {
        match port.write(&[self.number.as_u8() | Strobe::Address.as_u8()]) {
            Ok(_) => match port.read_exact(&mut []) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
    pub fn write(
        &mut self,
        port: &mut dyn serialport::SerialPort,
        data: u8,
    ) -> Result<(), std::io::Error> {
        match port.write(&[Strobe::Data.as_u8() | data]) {
            Ok(_) => match port.read_exact(&mut self.block_info) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn create() -> [Card; 8] {
        [
            Card {
                number: CardNo::One,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Two,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Three,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Four,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Five,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Six,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Seven,
                speed: 0,
                block_info: [0],
            },
            Card {
                number: CardNo::Fifteen,
                speed: 0,
                block_info: [0],
            },
        ]
    }
}
