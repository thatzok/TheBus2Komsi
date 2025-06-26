use crate::api::ApiVehicleType;
use crate::vehicle::VehicleState;
use crate::vehicle::init_vehicle_state;

pub fn get_vehicle_state_from_api(av: ApiVehicleType) -> VehicleState {
    let mut s = init_vehicle_state();

    match av.ignition_enabled.as_str() {
        "true" => s.ignition = 1,
        _ => s.ignition = 0,
    }

    match av.engine_started.as_str() {
        "true" => s.engine = 1,
        _ => s.engine = 0,
    }

    match av.warning_lights.as_str() {
        "true" => s.lights_warning = 1,
        _ => s.lights_warning = 0,
    }

    match av.passenger_doors_open.as_str() {
        "true" => s.doors = 1,
        _ => s.doors = 0,
    }

    match av.fixing_brake.as_str() {
        "true" => s.fixing_brake = 1,
        _ => s.fixing_brake = 0,
    }

    // wir prüfen nur ob gesetzt, nicht in welche Richtung (in api: -1,0,1 für links,aus,rechts)
    match av.indicator_state {
        0 => s.indicator = 0,  // aus
        -1 => s.indicator = 1, // an links
        1 => s.indicator = 2,  // an rechts
        _ => s.indicator = 0,
    }

    s.speed = av.speed.abs().round() as u32;
    s.maxspeed = av.allowed_speed.abs().round() as u32;

    s.fuel = (av.display_fuel * 100.0).trunc() as u32;

    s.lights_main = av.all_lamps.light_main.trunc() as u8;
    s.lights_high_beam = av.all_lamps.traveller_light.trunc() as u8;
    s.lights_front_door = av.all_lamps.front_door_light.trunc() as u8;
    s.lights_second_door = av.all_lamps.second_door_light.trunc() as u8;
    s.lights_stop_request = av.all_lamps.led_stop_request.trunc() as u8;
    s.lights_stop_brake = av.all_lamps.light_stopbrake.trunc() as u8;

    return s;
}
