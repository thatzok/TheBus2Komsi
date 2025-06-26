use std::fs;
use std::path::Path;
use the_bus_2_komsi::api::ApiVehicleType;
use serde_json;

#[test]
fn test_json_deserialization_from_files() {
    // Test files
    let ecitaro_file = Path::new("tests/ecitaro_12m_2d.json");
    let urbino_file = Path::new("tests/urbino_12m_2D.json");

    // Test ecitaro_12m_2d.json
    let ecitaro_json = fs::read_to_string(ecitaro_file)
        .expect("Failed to read ecitaro_12m_2d.json");

    let ecitaro_vehicle: ApiVehicleType = serde_json::from_str(&ecitaro_json)
        .expect("Failed to deserialize ecitaro_12m_2d.json");

    // Basic validation for ecitaro
    assert!(!ecitaro_vehicle.actor_name.is_empty(), "Actor name should not be empty");

    // Test urbino_12m_2D.json
    let urbino_json = fs::read_to_string(urbino_file)
        .expect("Failed to read urbino_12m_2D.json");

    let urbino_vehicle: ApiVehicleType = serde_json::from_str(&urbino_json)
        .expect("Failed to deserialize urbino_12m_2D.json");

    // Basic validation for urbino
    assert!(!urbino_vehicle.actor_name.is_empty(), "Actor name should not be empty");

    // Additional validations
    println!("Successfully deserialized ecitaro model: {}", ecitaro_vehicle.actor_name);
    println!("Successfully deserialized urbino model: {}", urbino_vehicle.actor_name);

    // Validate specific fields
    validate_vehicle(&ecitaro_vehicle, "Ecitaro");
    validate_vehicle(&urbino_vehicle, "Urbino");
}

fn validate_vehicle(vehicle: &ApiVehicleType, name: &str) {
    // Validate common fields that should be present in any vehicle
    assert!(vehicle.speed >= 0.0, "{} speed should be non-negative", name);
    assert!(vehicle.allowed_speed >= 0.0, "{} allowed speed should be non-negative", name);
    assert!(vehicle.display_fuel >= 0.0 && vehicle.display_fuel <= 100.0, 
            "{} fuel should be between 0 and 100", name);

    // Validate indicator state
    assert!((-1..=2).contains(&vehicle.indicator_state), 
            "{} indicator state should be between -1 and 2", name);

    // Validate lamps
    let lamps = &vehicle.all_lamps;
    assert!((0.0..=1.0).contains(&lamps.light_main), 
            "{} headlight should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.traveller_light), 
            "{} traveling light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.front_door_light), 
            "{} front door light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.second_door_light), 
            "{} second door light should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.led_stop_request), 
            "{} LED stop request should be between 0 and 1", name);
    assert!((0.0..=1.0).contains(&lamps.light_stopbrake), 
            "{} stop brake light should be between 0 and 1", name);
}
