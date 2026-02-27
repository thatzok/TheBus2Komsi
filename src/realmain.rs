use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Local;
use tokio::time::sleep;

use configparser::ini::Ini;
use komsi::komsi::{build_komsi_command, build_komsi_command_eol, KomsiCommand, KomsiDateTime};
use komsi::vehicle::{VehicleLogger, VehicleState};

struct PrintLogger;

impl VehicleLogger for PrintLogger {
    fn log(&self, msg: String) {
        println!("{}", msg);
    }
}
use serialport::SerialPort;

// TODO will be removed
use crate::opts::Opts;

use the_bus_telemetry::api::{get_current_vehicle_name, get_vehicle, get_world, RequestConfig};
use the_bus_telemetry::api2vehicle::get_vehicle_state_from_api;

// Serial port functionality
// This function is only included when the disablekomsiport feature is not enabled
#[cfg(not(feature = "disablekomsiport"))]
fn try_open_serial_port(
    portname: &str,
    baudrate: u32,
    verbose: bool,
) -> Option<Box<dyn SerialPort>> {
    match serialport::new(portname, baudrate).open() {
        Ok(port) => {
            if verbose {
                eprintln!("Port {:?} geöffnet mit {} baud.", portname, baudrate);
            }
            Some(port)
        }
        Err(e) => {
            eprintln!("Failed to open serial port {}: {}", portname, e);
            None
        }
    }
}

#[tokio::main]
pub async fn real_main(opts: &Opts) {
    let debug = opts.debug;
    let debug_serial = opts.debug_serial;
    let verbose = opts.verbose;

    if verbose {
        println!("Verbose Mode enabled.");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    }

    let mut vehicle_state = VehicleState::new();

    let config_path = "TheBus2Komsi.ini";

    let mut baudrate = 115200;
    let mut sleeptime = 200;
    let mut portnames = Vec::new();
    let mut clientip = "127.0.0.1".to_string();

    if Path::new(config_path).exists() {
        // now we get config ini
        let mut config_file = Ini::new();
        let _ = config_file.load(config_path);

        // Check for missing configuration values and use defaults if needed
        match config_file.getint("default", "baudrate") {
            Ok(Some(value)) => baudrate = value as u32,
            Ok(None) | Err(_) => {
                if verbose {
                    println!("Using default baudrate: {}", baudrate);
                }
            }
        }

        match config_file.getint("default", "sleeptime") {
            Ok(Some(value)) => sleeptime = value as u64,
            Ok(None) | Err(_) => {
                if verbose {
                    println!("Using default sleeptime: {}", sleeptime);
                }
            }
        }

        let port_keys = [
            "portname",
            "portname2",
            "portname3",
            "portname4",
            "portname5",
        ];
        for key in port_keys {
            if let Some(value) = config_file.get("default", key) {
                if !value.is_empty() {
                    portnames.push(value);
                }
            }
        }

        if portnames.is_empty() {
            if verbose {
                println!("Using default portname: COM1");
            }
            portnames.push("COM1".to_string());
        }

        match config_file.get("default", "ip") {
            Some(value) => clientip = value,
            None => {
                if verbose {
                    println!("Using default IP: {}", clientip);
                }
            }
        }
    } else if verbose {
        println!(
            "Config file {} not found, using default values IP: {}, portname: COM1, baudrate: {}, sleeptime: {}",
            config_path, clientip, baudrate, sleeptime
        );
        portnames.push("COM1".to_string());
    }

    // Display appropriate startup message based on feature configuration
    #[cfg(feature = "disablekomsiport")]
    println!("TheBus2Komsi has started. Have fun!");

    #[cfg(not(feature = "disablekomsiport"))]
    println!("TheBusTestAPI has started. Have fun!");

    // Serial port initialization and configuration
    // Create shared ports that can be safely accessed from multiple threads
    #[cfg(not(feature = "disablekomsiport"))]
    let ports = {
        let mut p_list = Vec::new();
        for pname in &portnames {
            p_list.push(Arc::new(Mutex::new(try_open_serial_port(
                pname, baudrate, verbose,
            ))));
        }
        p_list
    };

    #[cfg(not(feature = "disablekomsiport"))]
    let mut init_buffer = Vec::new();

    // Send SimulatorType:TheBus initialization message if port is available
    #[cfg(not(feature = "disablekomsiport"))]
    {
        init_buffer = Vec::new();

        let simulator_type = KomsiCommand::SimulatorType(1);
        let now = Local::now();
        use chrono::Datelike;
        use chrono::Timelike;
        let datetime = KomsiCommand::DateTime(KomsiDateTime {
            year: now.year() as u16,
            month: now.month() as u8,
            day: now.day() as u8,
            hour: now.hour() as u8,
            min: now.minute() as u8,
            sec: now.second() as u8,
        });

        // serialze simulator_type and datetime into buffer
        init_buffer.extend_from_slice(&build_komsi_command(simulator_type));
        init_buffer.extend_from_slice(&build_komsi_command(datetime));

        // hänge ein "\n" NEW-LINE an den Buffer
        init_buffer.extend_from_slice(&build_komsi_command_eol());


    };

    #[cfg(not(feature = "disablekomsiport"))]
    {
        for port in &ports {
            let mut port_guard = port.lock().unwrap();
            if let Some(ref mut p) = *port_guard {
                if let Err(e) = p.write(&init_buffer) {
                    eprintln!("Error writing to port: {}", e);
                }
            }
        }
    }

    // Prepare variables for the serial port reading threads
    #[cfg(not(feature = "disablekomsiport"))]
    let debug_serial_clone = debug_serial;
    #[cfg(not(feature = "disablekomsiport"))]
    let verbose_clone = verbose;

    // api client config struct
    let mut config = RequestConfig::new().host(clientip.clone()).debugging(debug);

    // Serial port reading threads
    // These threads continuously read data from the serial ports and handle reconnection if needed
    #[cfg(not(feature = "disablekomsiport"))]
    for (i, port) in ports.iter().enumerate() {
        let port_clone = Arc::clone(port);
        let portname_clone = portnames[i].clone();
        let init_buffer_clone = init_buffer.clone();
        thread::spawn(move || {
            loop {
                let mut need_reconnect = false;

                // Try to reconnect if port is not available
                {
                    let mut port_guard = port_clone.lock().unwrap();
                    if port_guard.is_none() {
                        *port_guard =
                            try_open_serial_port(&portname_clone, baudrate, verbose_clone);
                        // If reconnection successful, send SimulatorType:TheBus
                        if let Some(ref mut p) = *port_guard {
                            if let Err(e) = p.write(&init_buffer_clone) {
                                eprintln!(
                                    "Error writing to port {} after reconnection: {}",
                                    portname_clone, e
                                );
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
                                    eprint!("REC [{}]: ", portname_clone);
                                }

                                // Read available bytes
                                'reading: loop {
                                    match p.bytes_to_read() {
                                        Ok(bytes) if bytes > 0 => match p.read(&mut buffer) {
                                            Ok(bytes) => {
                                                if bytes > 0 && debug_serial_clone {
                                                    eprint!("{}", buffer[0] as char);
                                                }
                                            }
                                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                                            Err(e) => {
                                                eprintln!(
                                                    "Error reading from port {}: {:?}",
                                                    portname_clone, e
                                                );
                                                need_reconnect = true;
                                                break 'reading;
                                            }
                                        },
                                        Ok(_) => break 'reading,
                                        Err(e) => {
                                            eprintln!(
                                                "Error checking bytes to read on {}: {:?}",
                                                portname_clone, e
                                            );
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
                                eprintln!(
                                    "Error checking bytes to read on {}: {:?}",
                                    portname_clone, e
                                );
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
    }

    let sleeptime_error = 1000;

    let interval_error = Duration::from_millis(sleeptime_error);
    let interval = Duration::from_millis(sleeptime);

    let mut next_time = Instant::now() + interval;

    let mut vehicle_name = "".to_string();
    let mut old_vehicle_name = "".to_string();

    let mut zaehler = 0;
    let mut last_world_update = Instant::now() - Duration::from_secs(120); // Sofort beim ersten Mal ausführen

    loop {
        if (vehicle_name.is_empty()) || (zaehler > 10) {
            config.vehicle_name = "Current".to_string();
            vehicle_name = get_current_vehicle_name(&config).await;
            zaehler = 0;
        }

        if vehicle_name.is_empty() {
            println!("No vehicle found, not in bus.");
            vehicle_state = VehicleState::new();
            old_vehicle_name = "".to_string();
            sleep(interval_error).await;
            continue;
        }

        if config.debugging {
            println!("Vehicle-Name: {}", vehicle_name);
        }

        config.vehicle_name = vehicle_name.clone();

        let vehicle_response = get_vehicle(&config).await;
        if vehicle_response.is_err() {
            println!("Error getting vehicle data in JSON.");
            vehicle_name = "".to_string();
            sleep(interval_error).await;
            continue;
        }

        zaehler += 1;
        let vehicle = vehicle_response.unwrap();
        if config.vehicle_model != vehicle.vehicle_model {
            config.vehicle_model = vehicle.vehicle_model.clone();
        }

        if verbose && old_vehicle_name.is_empty() {
            println!("Hingesetzt. Jetzt gehts los!");
        }

        if vehicle_name != old_vehicle_name {
            if verbose {
                println!(
                    "Vehicle is now: model={} name={}",
                    config.vehicle_model, vehicle_name
                );
            }

            old_vehicle_name = vehicle_name.clone();
        }

        // now we can process

        let new_vehicle_state = get_vehicle_state_from_api(vehicle);
        if config.debugging {
            new_vehicle_state.print();
        }

        // compare and create cmd buf
        let mut logger: Option<&dyn VehicleLogger> = None;
        if verbose {
            logger = Some(&PrintLogger);
        }
        let mut cmdbuf = vehicle_state.compare(&new_vehicle_state, false, logger);

        // replace after compare for next round
        vehicle_state = new_vehicle_state;

        // ONLY every 2 minutes but only if we reach this point in the loop
        #[cfg(not(feature = "disablekomsiport"))]
        if last_world_update.elapsed() >= Duration::from_secs(120) {
            last_world_update = Instant::now();
            // now we check the world
            let api_world_response = get_world(&config).await;
            if api_world_response.is_err() {
                println!("Error getting world data.");
            } else {
                let date_time_str = api_world_response.unwrap().date_time;
                let komsi_date_time = KomsiDateTime::from_iso(&date_time_str);
                if komsi_date_time.is_ok() {
                    let komsi_date_time = komsi_date_time.unwrap();

                    let cmd_datetime = KomsiCommand::DateTime(komsi_date_time);

                    init_buffer = Vec::new(); // we use the init_buffer, but clear it here

                    let simulator_type = KomsiCommand::SimulatorType(1);
                    init_buffer.extend_from_slice(&build_komsi_command(simulator_type));
                    init_buffer.extend_from_slice(&build_komsi_command(cmd_datetime));

                    init_buffer.extend_from_slice(&build_komsi_command_eol());

                    // append to cmdbuf
                    cmdbuf.extend_from_slice(&init_buffer);
                }
            }
        }

        // Send commands to the serial ports when the disablekomsiport feature is not enabled
        #[cfg(not(feature = "disablekomsiport"))]
        if cmdbuf.len() > 0 {
            if opts.debug_serial {
                println!("SENDING -> {:?}", cmdbuf);
            }

            for (i, port) in ports.iter().enumerate() {
                // Write to serial port with reconnection handling
                let mut port_guard = port.lock().unwrap();

                // Try to reconnect if port is not available
                if port_guard.is_none() {
                    *port_guard = try_open_serial_port(&portnames[i], baudrate, verbose);
                }

                // Write to port if available
                if let Some(ref mut p) = *port_guard {
                    if let Err(e) = p.write(&cmdbuf) {
                        eprintln!("Error writing to port {}: {}", portnames[i], e);
                        // Port might be disconnected, set to None to trigger reconnection next time
                        *port_guard = None;
                    }
                }
            }
        }

        sleep(next_time - Instant::now()).await;
        next_time += interval;
    }
}
