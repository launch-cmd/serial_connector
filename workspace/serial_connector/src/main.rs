use std::time::Duration;

fn main() {
    test_serialport_lib();
}


fn test_serialport_lib() {
    port_scan();
    let port_path = String::from("/sys/class/tty/ttyS0");
    let port = open_port(port_path);
    match port {
        Some(mut port) => {
            let mut read_buf = Vec::new();
            write_to_port(port.as_mut(), String::from("testing"));
            read_from_port(port.as_mut(), read_buf.as_mut_slice());
            println!("Read from port: ");
            for ch in read_buf {
                print!("{}", ch);
            }
            println!("-------");
        }
        None => {}
    }
}

fn port_scan() {
    let ports = serialport::available_ports().expect("no ports found!");
    println!("Found ports: ");
    for p in ports {
        println!("{}", p.port_name);
    }
    println!("-------");
}

fn open_port(name: String) -> Option<std::boxed::Box<dyn serialport::SerialPort>> {
    match serialport::new(name, 9_600)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(port) => Some(port),
        Err(e) => {
            eprintln!("Error: {}", e.description);
            None
        }
    }
}

fn write_to_port(port: &mut dyn serialport::SerialPort, text: String) {
    port.write(text.as_bytes()).expect("Could not write!");
}

fn read_from_port(port: &mut dyn serialport::SerialPort, buffer: &mut [u8]) {
    port.read(buffer).expect("Could not read!");
}
