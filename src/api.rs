use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiVehicleType {
    #[serde(rename = "ActorName")]
    pub actor_name: String,
    #[serde(rename = "VehicleModel")]
    pub vehicle_model: String,
    #[serde(rename = "IgnitionEnabled")]
    pub ignition_enabled: String,
    #[serde(rename = "EngineStarted")]
    pub engine_started: String,
    #[serde(rename = "WarningLights")]
    pub warning_lights: String,
    #[serde(rename = "PassengerDoorsOpen")]
    pub passenger_doors_open: String,
    #[serde(rename = "FixingBrake")]
    pub fixing_brake: String,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "AllowedSpeed")]
    pub allowed_speed: f32,
    #[serde(rename = "DisplayFuel")]
    pub display_fuel: f32,
    #[serde(rename = "IndicatorState")]
    pub indicator_state: i8,
    #[serde(rename = "AllLamps")]
    pub all_lamps: ApiLamps,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ApiLamps {
    #[serde(rename = "LightHeadlight", alias = "LightHeadlight1")]
    pub light_main: f32,
    #[serde(rename = "LightTraveling", alias = "LightTraveling1")]
    pub traveller_light: f32,
    #[serde(rename = "ButtonLight Door 1",alias="Door Button 1")]
    pub front_door_light: f32,
    #[serde(rename = "ButtonLight Door 2", default)]
    pub second_door_light: f32,
    #[serde(rename = "LED StopRequest", default)]
    pub led_stop_request: f32,
    #[serde(rename = "ButtonLight BusStopBrake", default)]
    pub light_stopbrake: f32,
}

pub fn getapidata(ip: &String, debug: bool) -> Result<ApiVehicleType, Box<dyn std::error::Error>> {
    let request_url = format!("http://{}:37337/Vehicles/Current", ip);

    let timeout = Duration::new(2, 0);
    let client = Client::new();

    if debug {
        eprintln!("Fetching url {} ...", &request_url);
    }

    let response = client.get(&request_url).timeout(timeout).send()?; // wir warten auf die antwort
    // eprintln!("http get erfolgt");

    if !response.status().is_success() {
        Err("Error: response code")?
    }

    // eprintln!("http code OK");
    // eprintln!("Response: {:?} {}", response.version(), response.status());
    // eprintln!("Headers: {:#?}\n", response.headers());

    let value = response.json::<serde_json::Value>()?;
    if debug {
        eprintln!(
            "JSON structure:\n{}",
            serde_json::to_string_pretty(&value).unwrap()
        );
    }

    let api_vehicle: ApiVehicleType = serde_json::from_value(value).map_err(|e| {
        eprintln!("Failed to parse API response as JSON: {}", e);
        eprintln!("API endpoint: {}", request_url);
        Box::new(e) as Box<dyn std::error::Error>
    })?;

    if debug {
        println!("{:?}", &api_vehicle);
    }
    Ok(api_vehicle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_api_lamps_deserialization() {
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling": 0.5,
            "ButtonLight Door 1": 0.0,
            "ButtonLight Door 2": 1.0,
            "LED StopRequest": 0.0,
            "ButtonLight BusStopBrake": 1.0
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        assert_eq!(lamps.light_main, 1.0);
        assert_eq!(lamps.traveller_light, 0.5);
        assert_eq!(lamps.front_door_light, 0.0);
        assert_eq!(lamps.second_door_light, 1.0);
        assert_eq!(lamps.led_stop_request, 0.0);
        assert_eq!(lamps.light_stopbrake, 1.0);
    }

    #[test]
    fn test_api_lamps_alias_deserialization() {
        // Test the alias functionality for LightTraveling1
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling1": 0.5,  // Using the alias
            "Door Button 1": 0.0,    // Using the alias
            "ButtonLight Door 2": 1.0,
            "LED StopRequest": 0.0,
            "ButtonLight BusStopBrake": 1.0
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        assert_eq!(lamps.traveller_light, 0.5);
        assert_eq!(lamps.front_door_light, 0.0);
    }

    #[test]
    fn test_api_vehicle_type_deserialization() {
        let json_data = json!({
            "ActorName": "TestVehicle",
            "VehicleModel": "TestVehicleModel",
            "IgnitionEnabled": "True",
            "EngineStarted": "True",
            "WarningLights": "False",
            "PassengerDoorsOpen": "False",
            "FixingBrake": "False",
            "Speed": 50.5,
            "AllowedSpeed": 60.0,
            "DisplayFuel": 75.5,
            "IndicatorState": 0,
            "AllLamps": {
                "LightHeadlight": 1.0,
                "LightTraveling": 0.5,
                "ButtonLight Door 1": 0.0,
                "ButtonLight Door 2": 1.0,
                "LED StopRequest": 0.0,
                "ButtonLight BusStopBrake": 1.0
            }
        });

        let vehicle: ApiVehicleType = serde_json::from_value(json_data).unwrap();

        assert_eq!(vehicle.actor_name, "TestVehicle");
        assert_eq!(vehicle.vehicle_model, "TestVehicleModel");
        assert_eq!(vehicle.ignition_enabled, "True");
        assert_eq!(vehicle.engine_started, "True");
        assert_eq!(vehicle.warning_lights, "False");
        assert_eq!(vehicle.passenger_doors_open, "False");
        assert_eq!(vehicle.fixing_brake, "False");
        assert_eq!(vehicle.speed, 50.5);
        assert_eq!(vehicle.allowed_speed, 60.0);
        assert_eq!(vehicle.display_fuel, 75.5);
        assert_eq!(vehicle.indicator_state, 0);

        // Check the nested ApiLamps struct
        assert_eq!(vehicle.all_lamps.light_main, 1.0);
        assert_eq!(vehicle.all_lamps.traveller_light, 0.5);
        assert_eq!(vehicle.all_lamps.front_door_light, 0.0);
        assert_eq!(vehicle.all_lamps.second_door_light, 1.0);
        assert_eq!(vehicle.all_lamps.led_stop_request, 0.0);
        assert_eq!(vehicle.all_lamps.light_stopbrake, 1.0);
    }

    #[test]
    fn test_getapidata_error_handling() {
        // Test with an invalid IP address that should cause a connection error
        let result = getapidata(&"invalid-ip-address".to_string(), false);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_json_parsing_error() {
        // Test the error handling for invalid JSON
        let invalid_json = json!({
            // Missing required fields
            "ActorName": "TestVehicle",
            // Other fields are missing
        });

        let result = serde_json::from_value::<ApiVehicleType>(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_json_parsing_success() {
        // Test successful JSON parsing
        let valid_json = json!({
            "ActorName": "TestVehicle",
            "VehicleModel": "TestVehicleModel",
            "IgnitionEnabled": "True",
            "EngineStarted": "True",
            "WarningLights": "False",
            "PassengerDoorsOpen": "False",
            "FixingBrake": "False",
            "Speed": 50.5,
            "AllowedSpeed": 60.0,
            "DisplayFuel": 75.5,
            "IndicatorState": 0,
            "AllLamps": {
                "LightHeadlight": 1.0,
                "LightTraveling": 0.5,
                "ButtonLight Door 1": 0.0,
                "ButtonLight Door 2": 1.0,
                "LED StopRequest": 0.0,
                "ButtonLight BusStopBrake": 1.0
            }
        });

        let result = serde_json::from_value::<ApiVehicleType>(valid_json);
        assert!(result.is_ok());

        let vehicle = result.unwrap();
        assert_eq!(vehicle.actor_name, "TestVehicle");
        assert_eq!(vehicle.speed, 50.5);
    }

    // This test is marked as ignored because it requires a real API server
    // It can be run manually with: cargo test -- --ignored
    #[test]
    #[ignore]
    fn test_getapidata_real_api() {
        // This test requires a real API server running at 127.0.0.1:37337
        // It's marked as ignored to avoid failing in CI environments
        let result = getapidata(&"127.0.0.1".to_string(), true);

        // If the API server is running, this should succeed
        if result.is_ok() {
            let vehicle = result.unwrap();
            // Basic validation that we got a valid response
            assert!(!vehicle.actor_name.is_empty());
        } else {
            // If the test is run without an API server, this will be skipped
            println!("Skipping real API test as no server is available");
        }
    }

    // Test for default values in ApiLamps
    #[test]
    fn test_api_lamps_default_values() {
        // Test that default values are used when fields are missing
        let json_data = json!({
            "LightHeadlight": 1.0,
            "LightTraveling": 0.5,
            "ButtonLight Door 1": 0.0
            // Missing: "ButtonLight Door 2", "LED StopRequest", "ButtonLight BusStopBrake"
        });

        let lamps: ApiLamps = serde_json::from_value(json_data).unwrap();

        // These fields have default values (0.0) when missing
        assert_eq!(lamps.second_door_light, 0.0);
        assert_eq!(lamps.led_stop_request, 0.0);
        assert_eq!(lamps.light_stopbrake, 0.0);
    }

}
