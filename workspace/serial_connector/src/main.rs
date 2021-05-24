mod card;
mod card_no;
mod strobe;
use card::*;
use card_no::*;
use strobe::*;

fn main() {

    //create list of all cards.
    let mut cards = Card::create();

    //usb port setup
    let port_path = String::from("/dev/ttyUSB0");
    let baud_rate = 9600;
    let timeout_ms = 200;
    let mut port = open_port(port_path, baud_rate, timeout_ms).expect("Could not open port!");
    prepare_port(port.as_mut()).expect("Could not prepare port!");
    
    //start main loop when usb port is opened succesfully.
    program_loop(port.as_mut(), &mut cards);
}

fn program_loop(port: &mut dyn serialport::SerialPort, cards: &mut [Card; 8]) {
    loop {
        for card in &mut *cards {
            if card.number != CardNo::Fifteen {
                card.select(port)
                    .and_then(|_| card.write(port, card.number.as_u8()))
                    .unwrap_or_else(|error| {
                        eprintln!("Error during r/w operation: {}", error);
                    })
            }
        }
    }
}

fn open_port(
    port_path: String,
    baud_rate: u32,
    timeout_ms: u64,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    serialport::new(port_path, baud_rate)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .baud_rate(baud_rate)
        .parity(serialport::Parity::None)
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .open()
}

fn prepare_port(port: &mut dyn serialport::SerialPort) -> std::option::Option<()> {
    port.write_data_terminal_ready(true)
        .ok()
        .and_then(|_| port.flush().ok())
        .and_then(|_| {
            port.write(&[CardNo::One.as_u8() | Strobe::Address.as_u8()])
                .ok()
        })
        .and_then(|_| port.read_exact(&mut []).ok())
    }
