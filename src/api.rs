use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct ApiVehicleType {
    #[serde(rename = "ActorName")]
    pub actor_name: String,
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

#[derive(Deserialize, Debug)]
pub struct ApiLamps {
    #[serde(rename = "LightHeadlight")]
    pub light_main: f32,
    #[serde(rename = "LightTraveling")]
    pub traveller_light: f32,
    #[serde(rename = "ButtonLight Door 1")]
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

    let api_vehicle: ApiVehicleType = response.json()?;
    if debug {
        println!("{:?}", &api_vehicle);
    }
    Ok(api_vehicle)
}
