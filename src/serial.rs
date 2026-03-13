use serialport::{SerialPortType, available_ports};
use nusb;

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


pub fn show_precise_com_ports() {
    let usb_devices: Vec<_> = nusb::list_devices()
        .map(|iter| iter.collect::<Vec<_>>())
        .unwrap_or_default();

    let ports = available_ports().unwrap_or_default();

    println!("{:<8} | {:<20} | {:<30} | {:<22} | {}", "Port", "Hersteller", "Produkt", "Seriennummer", "Version");
    println!("{:-<115}", "");

    for p in ports {
        if let SerialPortType::UsbPort(info) = p.port_type {
            let real_usb = usb_devices.iter().find(|d| {
                let ids_match = d.vendor_id() == info.vid && d.product_id() == info.pid;
                let sn_match = match (&d.serial_number(), &info.serial_number) {
                    (Some(u_sn), Some(p_sn)) => {
                        let u = u_sn.to_uppercase();
                        let p = p_sn.to_uppercase();
                        p.contains(&u) || u.contains(&p)
                    },
                    _ => true, // Wenn eine SN fehlt, matchen wir nur über VID/PID
                };
                ids_match && sn_match
            });

            let chip_mfr = real_usb.and_then(|d| d.manufacturer_string()).unwrap_or("");
            let chip_prd = real_usb.and_then(|d| d.product_string()).unwrap_or("");
            let chip_sn  = real_usb.and_then(|d| d.serial_number()).unwrap_or("");
            let chip_ver = real_usb.map(|d| d.device_version()).unwrap_or(0);

            let reg_mfr = info.manufacturer.as_deref().unwrap_or("");
            let reg_prd = info.product.as_deref().unwrap_or("");
            let reg_sn  = info.serial_number.as_deref().unwrap_or("");

            let mfr = if !chip_mfr.is_empty() && !chip_mfr.to_lowercase().contains("microsoft") {
                chip_mfr
            } else if !reg_mfr.is_empty() {
                reg_mfr
            } else {
                "Unbekannt"
            };

            let mut prd = if !chip_prd.is_empty() && !chip_prd.contains("USB Single Serial") {
                chip_prd.to_string()
            } else {
                reg_prd.to_string()
            };

            if let Some(pos) = prd.find(" (COM") {
                prd.truncate(pos);
            }

            let sn = if !chip_sn.is_empty() { chip_sn } else { reg_sn };

            let version_str = if chip_ver > 0 {
                format!("v{:x}.{:02x}", (chip_ver >> 8), (chip_ver & 0xFF))
            } else {
                "n/a".to_string()
            };

            println!(
                "{:<8} | {:<20} | {:<30} | {:<22} | {}",
                p.port_name, mfr, prd, sn, version_str
            );
        }
    }
}
