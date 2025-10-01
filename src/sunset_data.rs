use chrono::NaiveTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SunriseSunsetResults {
    pub date: Option<String>,
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
    pub utc_offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SunriseSunsetResponse {
    pub results: SunriseSunsetResults,
    pub status: String,
}

pub fn fetch_sunrise_sunset(
    lat: f32,
    long: f32,
) -> Result<SunriseSunsetResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://apiz.sunrisesunset.io/json?lat={:.6}&lng={:.6}",
        lat, long
    );
    println!("Fetching sunrise/sunset data from: {}", url);
    let result = reqwest::blocking::get(&url)?;
    let resp = result.json::<SunriseSunsetResponse>()?;
    Ok(resp)
}

pub fn get_sunrise_sunset_for_today(
    lat: f32,
    long: f32,
) -> Result<(NaiveTime, NaiveTime), Box<dyn std::error::Error>> {
    const FORMAT_TIME: &str = "%I:%M:%S %p";

    let data = fetch_sunrise_sunset(lat, long)?;

    let time_sunrise = NaiveTime::parse_from_str(&data.results.sunrise.unwrap(), FORMAT_TIME)?;
    let time_sunset = NaiveTime::parse_from_str(&data.results.sunset.unwrap(), FORMAT_TIME)?;

    Ok((time_sunrise, time_sunset))
}
