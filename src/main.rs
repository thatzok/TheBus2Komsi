mod api;
mod komsi;
mod opts;
mod serial;
mod vehicle;

use std::io;
use std::io::Read;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

use crate::api::getapidata;
use crate::opts::Opts;
use crate::serial::show_serial_comports;
use crate::vehicle::compare_vehicle_states;
use crate::vehicle::get_vehicle_state;
use crate::vehicle::init_vehicle_state;
use crate::vehicle::print_vehicle_state;

fn main() {
    let opts = Opts::from_args();

    let mut port_name = String::from("");

    if opts.debug {
        println!("{:?}", opts);
    }

    if opts.list {
        show_serial_comports();
        return;
    }


    if opts.port.is_some() {
        port_name = <std::option::Option<std::string::String> as Clone>::clone(&opts.port).unwrap();
    }

    if  (port_name.len() < 1) {
        println!("Kein COMport angegeben.");
        return;
    }

    if opts.clear {
        println!("send empty vehicle data to comport");

        let empty_vehicle = init_vehicle_state();
        let vec = compare_vehicle_states(&empty_vehicle, &empty_vehicle, &opts, true);
        if opts.debug_serial {
            println!("SENDING -> {:?}", vec);
        }

            let baud_rate = opts.baud;

            let mut port = serialport::new(&port_name, baud_rate)
                .open()
                .expect("Failed to open serial port");

            if opts.verbose {
                eprintln!("Port {:?} geöffnet mit {} baud.", &port_name, &baud_rate);
            }

            let _ = port.write(&vec);

        return;
    }

    // default, wenn keine anderen Optionen ausgewählt,
        real_main(port_name, &opts);

}

fn real_main(port_name: String, opts: &Opts) {
    let debug = opts.debug;
    let debug_serial = opts.debug_serial;

    let verbose = opts.verbose;

    let mut vehicle_state = init_vehicle_state();

    let mut api_state = -1;

    let baud_rate = opts.baud;

    let mut port = serialport::new(&port_name, baud_rate)
        .open()
        .expect("Failed to open serial port");

    if verbose {
        eprintln!("Port {:?} geöffnet mit {} baud.", &port_name, &baud_rate);
    }

    
    // send SimulatorType:TheBus
    let string = "O1\x0a";
    let buffer = string.as_bytes();
    let _ = port.write(buffer);

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");

    // empfang über seriell ist ausgelagert in eigenen thread
    thread::spawn(move || loop {
        // Read the bytes back from the cloned port
        let mut buffer: [u8; 1] = [0; 1];

        if clone.bytes_to_read().unwrap() > 0 {
            if debug_serial {
                eprint!("REC: ");
            }

            while clone.bytes_to_read().unwrap() > 0 {
                match clone.read(&mut buffer) {
                    Ok(bytes) => {
                        if bytes > 0 {
                            if debug_serial {
                                eprint!("{}", buffer[0] as char);
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            if debug_serial {
                eprintln!("");
            }
        }

        thread::sleep(Duration::from_millis(100));
    });

    let interval = Duration::from_millis(opts.sleeptime);
    let mut next_time = Instant::now() + interval;

    loop {
        let api_bus_result = getapidata(&opts);

        if api_bus_result.is_err() {
            // eprintln!("getapidata error: {}", api_bus_result.unwrap_err());
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

            let newstate = get_vehicle_state(api_bus);
            if debug {
                print_vehicle_state(&newstate);
            }

            // compare and create cmd buf
            let cmdbuf = compare_vehicle_states(&vehicle_state, &newstate, &opts, false);

            // replace after compare for next round
            vehicle_state = newstate;

            if cmdbuf.len() > 0 {
                if opts.debug_serial {
                    println!("SENDING -> {:?}", cmdbuf);
                }

                // Write to serial port
                let _ = port.write(&cmdbuf);
            }
        }

        sleep(next_time - Instant::now());
        next_time += interval;
    }
}
