use serialport::{available_ports, SerialPortType};

pub fn show_serial_comports() {
    match available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("Kein port gefunden."),
                1 => println!("1 port gefunden:"),
                n => println!("{} ports gefunden:", n),
            };

            for p in ports {
                print!("  {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        print!(" Typ: USB");
                        print!(
                            "   Hersteller: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "   Produkt: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("    Typ: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("    Typ: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("    Typ: Unknown");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("Error listing serial ports");
        }
    }
}

