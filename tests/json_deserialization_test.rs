use std::fs;
use std::path::Path;
use the_bus_2_komsi::api::ApiVehicleType;
use serde_json;

#[test]
fn test_json_deserialization_from_files() {

//    test_vehicle_deserialization("tests/json/man_lionscity.json", "BLA");
    test_vehicle_deserialization("tests/json/mb_ecitaro.json", "BP_Mercedes_eCitaro_12m_2Door_C_2147345014");
    test_vehicle_deserialization("tests/json/scania_citywide.json", "BP_Scania_Citywide_12M2D_C_2147248282");
    test_vehicle_deserialization("tests/json/solaris_urbino.json", "BP_Solaris_Urbino_12m_2D_C_2147468046");
    test_vehicle_deserialization("tests/json/vdl_citea.json", "BP_VDL_Citea_LLE_120_2D_C_2147124848");
    
}

fn test_vehicle_deserialization(file_path: &str, actor_name: &str) {
    // Create path
    let file = Path::new(file_path);

    // Read and deserialize the JSON file
    let json = fs::read_to_string(file)
        .expect(&format!("Failed to read {}", file_path));

    let vehicle: ApiVehicleType = serde_json::from_str(&json)
        .expect(&format!("Failed to deserialize {}", file_path));

    // Basic validation
    assert!(!vehicle.actor_name.is_empty(), "Actor name should not be empty");
    assert_eq!(vehicle.actor_name, actor_name, "Actor name should match the expected value");

    // Print success message
    println!("Successfully deserialized \"{}\" model: {}", vehicle.vehicle_model, vehicle.actor_name);

    // Validate specific fields
    validate_vehicle(&vehicle, actor_name);
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
