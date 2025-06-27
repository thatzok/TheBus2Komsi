use std::io;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use configparser::ini::Ini;
use serialport::SerialPort;

use crate::api::getapidata;
use crate::api2vehicle::get_vehicle_state_from_api;
use crate::opts::Opts;
use crate::vehicle::compare_vehicle_states;
use crate::vehicle::init_vehicle_state;
use crate::vehicle::print_vehicle_state;


// Function to attempt to open the serial port
#[cfg(not(feature = "disablekomsiport"))]
fn try_open_serial_port(portname: &str, baudrate: u32, verbose: bool) -> Option<Box<dyn SerialPort>> {
    match serialport::new(portname, baudrate).open() {
        Ok(port) => {
            if verbose {
                eprintln!("Port {:?} geöffnet mit {} baud.", portname, baudrate);
            }
            Some(port)
        },
        Err(e) => {
            eprintln!("Failed to open serial port {}: {}", portname, e);
            None
        }
    }
}

pub fn real_main(opts: &Opts) {
    let debug = opts.debug;
    let debug_serial = opts.debug_serial;
    let verbose = opts.verbose;

    if verbose {
        println!("Verbose Mode enabled.");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    }

    let mut vehicle_state = init_vehicle_state();
    let mut api_state = -1;

    // TODO checking for file not found and elements not found
    let config_path = "TheBus2Komsi.ini";

    let mut baudrate = 115200;
    let mut sleeptime = 1000;
    let mut portname = "COM1".to_string();
    let mut clientip = "127.0.0.1".to_string();

    if Path::new(config_path).exists() {
        // now we get config ini
        let mut config = Ini::new();
        let _ = config.load(config_path);

        baudrate = config.getint("default", "baudrate").unwrap().unwrap() as u32;
        sleeptime = config.getint("default", "sleeptime").unwrap().unwrap() as u64;
        portname = config.get("default", "portname").unwrap();
        clientip = config.get("default", "ip").unwrap();
    }

    #[cfg(feature = "disablekomsiport")]
    println!("TheBus2Komsi has started. Have fun!");

    #[cfg(not(feature = "disablekomsiport"))]
    println!("TheBusTestAPI has started. Have fun!");

    // Create a shared port that can be safely accessed from multiple threads
    #[cfg(not(feature = "disablekomsiport"))]
    let port = Arc::new(Mutex::new(try_open_serial_port(&portname, baudrate, verbose)));

    // send SimulatorType:TheBus if port is available
    #[cfg(not(feature = "disablekomsiport"))]    
    let string = "O1\x0a";
    #[cfg(not(feature = "disablekomsiport"))]    
    let buffer = string.as_bytes();
    #[cfg(not(feature = "disablekomsiport"))]
    {
        let mut port_guard = port.lock().unwrap();
        if let Some(ref mut p) = *port_guard {
            if let Err(e) = p.write(buffer) {
                eprintln!("Error writing to port: {}", e);
            }
        }
    }

    // Clone the port for the reading thread
    #[cfg(not(feature = "disablekomsiport"))]
    let port_clone = Arc::clone(&port);
    #[cfg(not(feature = "disablekomsiport"))]
    let portname_clone = portname.clone();
    #[cfg(not(feature = "disablekomsiport"))]
    let debug_serial_clone = debug_serial;
    #[cfg(not(feature = "disablekomsiport"))]
    let verbose_clone = verbose;

    // empfang über seriell ist ausgelagert in eigenen thread
    #[cfg(not(feature = "disablekomsiport"))]    
    thread::spawn(move || {
        loop {
            let mut need_reconnect = false;

            // Try to reconnect if port is not available
            {
                let mut port_guard = port_clone.lock().unwrap();
                if port_guard.is_none() {
                    *port_guard = try_open_serial_port(&portname_clone, baudrate, verbose_clone);
                    // If reconnection successful, send SimulatorType:TheBus
                    if let Some(ref mut p) = *port_guard {
                        if let Err(e) = p.write(buffer) {
                            eprintln!("Error writing to port after reconnection: {}", e);
                            // Mark for reconnection on next iteration
                            *port_guard = None;
                        }
                    }
                }
            }

            // Read the bytes back from the port
            let mut buffer: [u8; 1] = [0; 1];

            // Scope for port_guard to ensure it's dropped before we try to reconnect
            {
                let mut port_guard = port_clone.lock().unwrap();

                if let Some(ref mut p) = *port_guard {
                    // Check if there are bytes to read
                    match p.bytes_to_read() {
                        Ok(bytes) if bytes > 0 => {
                            if debug_serial_clone {
                                eprint!("REC: ");
                            }

                            // Read available bytes
                            'reading: loop {
                                match p.bytes_to_read() {
                                    Ok(bytes) if bytes > 0 => {
                                        match p.read(&mut buffer) {
                                            Ok(bytes) => {
                                                if bytes > 0 && debug_serial_clone {
                                                    eprint!("{}", buffer[0] as char);
                                                }
                                            }
                                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                            Err(e) => {
                                                eprintln!("Error reading from port: {:?}", e);
                                                need_reconnect = true;
                                                break 'reading;
                                            }
                                        }
                                    }
                                    Ok(_) => break 'reading,
                                    Err(e) => {
                                        eprintln!("Error checking bytes to read: {:?}", e);
                                        need_reconnect = true;
                                        break 'reading;
                                    }
                                }
                            }

                            if debug_serial_clone {
                                eprintln!("");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking bytes to read: {:?}", e);
                            need_reconnect = true;
                        }
                        _ => {}
                    }
                }

                // If we need to reconnect, set the port to None
                if need_reconnect {
                    *port_guard = None;
                }
            }

            // Sleep before next iteration
            thread::sleep(Duration::from_millis(100));
        }
    });

    let interval = Duration::from_millis(sleeptime);
    let mut next_time = Instant::now() + interval;

    loop {
        let api_bus_result = getapidata(&clientip, opts.debug);

        if api_bus_result.is_err() {
            if debug {
                eprintln!("getapidata error: {}", api_bus_result.unwrap_err());
            }
            if api_state != 0 {
                if verbose {
                    println!("Bitte einsteigen und hinsetzen.");
                }
                api_state = 0;
            }
        } else {
            let api_bus = api_bus_result.unwrap();
            // println!("{:?}", api_bus);
            if api_state != 1 {
                if verbose {
                    println!("Hingesetzt. Jetzt gehts los!");
                }
                api_state = 1;
            }

            let newstate = get_vehicle_state_from_api(api_bus);
            if debug {
                print_vehicle_state(&newstate);
            }

            // compare and create cmd buf
            let cmdbuf = compare_vehicle_states(&vehicle_state, &newstate, verbose, false);

            // replace after compare for next round
            vehicle_state = newstate;

            #[cfg(not(feature = "disablekomsiport"))]
            if cmdbuf.len() > 0 {
                if opts.debug_serial {
                    println!("SENDING -> {:?}", cmdbuf);
                }

                // Write to serial port
                let mut port_guard = port.lock().unwrap();
                // Try to reconnect if port is not available
                if port_guard.is_none() {
                    *port_guard = try_open_serial_port(&portname, baudrate, verbose);
                }

                // Write to port if available
                if let Some(ref mut p) = *port_guard {
                    if let Err(e) = p.write(&cmdbuf) {
                        eprintln!("Error writing to port: {}", e);
                        // Port might be disconnected, set to None to trigger reconnection next time
                        *port_guard = None;
                    }
                }
            }
        }

        sleep(next_time - Instant::now());
        next_time += interval;
    }
}
