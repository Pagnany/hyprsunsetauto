//https://api.sunrisesunset.io/json?lat=38.907192&lng=-77.036873

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SunriseSunsetResults {
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub first_light: Option<String>,
    pub last_light: Option<String>,
    pub dawn: Option<String>,
    pub dusk: Option<String>,
    pub solar_noon: Option<String>,
    pub golden_hour: Option<String>,
    pub day_length: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SunriseSunsetResponse {
    pub results: SunriseSunsetResults,
    pub status: String,
}

pub fn fetch_sunrise_sunset(
    lat: f64,
    lng: f64,
) -> Result<SunriseSunsetResponse, Box<dyn std::error::Error>> {
    let url = format!("https://api.sunrisesunset.io/json?lat={}&lng={}", lat, lng);
    let result = reqwest::blocking::get(&url)?;
    let resp = result.json::<SunriseSunsetResponse>()?;
    Ok(resp)
}
