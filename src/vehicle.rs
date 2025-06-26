use crate::komsi::KomsiCommandKind;
use crate::komsi::build_komsi_command;
use crate::komsi::build_komsi_command_u8;
use crate::komsi::build_komsi_command_eol;

#[derive(Debug)]
pub struct VehicleState {
    pub ignition: u8,
    pub engine: u8,
    pub doors: u8,
    pub speed: u32,
    pub maxspeed: u32,
    pub fuel: u32,
    pub indicator: u8,
    pub fixing_brake: u8,
    pub lights_warning: u8,
    pub lights_main: u8,
    pub lights_front_door: u8,
    pub lights_second_door: u8,
    pub lights_stop_request: u8,
    pub lights_stop_brake: u8,
    pub lights_high_beam: u8,
    pub battery_light: u8,
}

pub fn print_vehicle_state(v: &VehicleState) {
    print!("ignition:{} ", v.ignition);
    print!("engine:{} ", v.engine);
    print!("indicator:{} ", v.indicator);
    print!("fuel:{} ", v.fuel);
    print!("warn:{} ", v.lights_warning);
    print!("lights:{} ", v.lights_main);
    print!("lights-highbeam:{} ", v.lights_high_beam);
    print!("stop:{} ", v.lights_stop_request);
    print!("fixingbrake:{} ", v.fixing_brake);
    print!("stopbrake:{} ", v.lights_stop_brake);
    print!("doors:{} ", v.doors);
    print!("door1:{} ", v.lights_front_door);
    print!("door2:{} ", v.lights_second_door);
    print!("speed:{} ", v.speed);
    print!("maxspeed:{} ", v.maxspeed);
    print!("batterylight:{} ", v.battery_light);
    println!(" ");
}

pub fn init_vehicle_state() -> VehicleState {
    let s = VehicleState {
        ignition: 0,
        engine: 0,
        doors: 0,
        speed: 0,
        indicator: 0,
        fixing_brake: 0,
        lights_warning: 0,
        lights_main: 0,
        lights_front_door: 0,
        lights_second_door: 0,
        lights_stop_request: 0,
        maxspeed: 0,
        lights_high_beam: 0,
        fuel: 0,
        lights_stop_brake: 0,
        battery_light: 0,
    };
    return s;
}


// Helper function for handling u8 field changes
fn handle_u8_field_change(
    old_value: u8,
    new_value: u8,
    field_name: &str,
    command_kind: KomsiCommandKind,
    verbose: bool,
    force: bool,
    buffer: &mut Vec<u8>,
) {
    if (old_value != new_value) || force {
        if verbose {
            println!("{}: {} -> {} ", field_name, old_value, new_value);
        }
        let mut b = build_komsi_command_u8(command_kind, new_value);
        buffer.append(&mut b);
    }
}

// Helper function for handling u32 field changes
fn handle_u32_field_change(
    old_value: u32,
    new_value: u32,
    field_name: &str,
    command_kind: KomsiCommandKind,
    verbose: bool,
    force: bool,
    buffer: &mut Vec<u8>,
) {
    if (old_value != new_value) || force {
        if verbose {
            println!("{}:  {} -> {} ", field_name, old_value, new_value);
        }
        let mut b = build_komsi_command(command_kind, new_value);
        buffer.append(&mut b);
    }
}

pub fn compare_vehicle_states(
    old: &VehicleState,
    new: &VehicleState,
    verbose: bool,
    force: bool,
) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; 0];

    // Handle u8 fields
    handle_u8_field_change(old.ignition, new.ignition, "ignition", KomsiCommandKind::Ignition, verbose, force, &mut buffer);
    handle_u8_field_change(old.engine, new.engine, "engine", KomsiCommandKind::Engine, verbose, force, &mut buffer);
    handle_u8_field_change(old.doors, new.doors, "doors", KomsiCommandKind::PassengerDoorsOpen, verbose, force, &mut buffer);
    handle_u8_field_change(old.fixing_brake, new.fixing_brake, "fixing_brake", KomsiCommandKind::FixingBrake, verbose, force, &mut buffer);
    handle_u8_field_change(old.indicator, new.indicator, "indicator", KomsiCommandKind::Indicator, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_warning, new.lights_warning, "lights_warning", KomsiCommandKind::LightsWarning, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_main, new.lights_main, "lights_main", KomsiCommandKind::LightsMain, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_stop_request, new.lights_stop_request, "lights_stop_request", KomsiCommandKind::LightsStopRequest, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_stop_brake, new.lights_stop_brake, "lights_stop_brake", KomsiCommandKind::LightsStopBrake, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_front_door, new.lights_front_door, "lights_front_door", KomsiCommandKind::LightsFrontDoor, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_second_door, new.lights_second_door, "lights_second_door", KomsiCommandKind::LightsSecondDoor, verbose, force, &mut buffer);
    handle_u8_field_change(old.lights_high_beam, new.lights_high_beam, "lights_high_beam", KomsiCommandKind::LightsHighBeam, verbose, force, &mut buffer);
    handle_u8_field_change(old.battery_light, new.battery_light, "batterylight", KomsiCommandKind::BatteryLight, verbose, force, &mut buffer);

    // Handle u32 fields
    handle_u32_field_change(old.fuel, new.fuel, "fuel", KomsiCommandKind::Fuel, verbose, force, &mut buffer);
    handle_u32_field_change(old.speed, new.speed, "speed", KomsiCommandKind::Speed, verbose, force, &mut buffer);
    handle_u32_field_change(old.maxspeed, new.maxspeed, "maxspeed", KomsiCommandKind::MaxSpeed, verbose, force, &mut buffer);

    // Add end of line if buffer is not empty
    if buffer.len() > 0 {
        let mut b = build_komsi_command_eol();
        buffer.append(&mut b);
    }

    return buffer;
}
