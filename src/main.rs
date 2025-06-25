use std::process::Command;
use std::{thread, time};

pub mod sunset_data;

fn main() {
    // get parameters
    // cargo r -- 53.5 9.7 6500 3500
    let args: Vec<String> = std::env::args().collect();
    let lat: f32 = args[1].parse().unwrap_or(53.5);
    let long: f32 = args[2].parse().unwrap_or(9.7);
    let temperature_day: i32 = args[3].parse().unwrap_or(6500);
    let temperature_night: i32 = args[4].parse().unwrap_or(3500);

    if let Err(e) = toggle_temperature(temperature_day, temperature_night) {
        eprintln!("Error: {}", e);
    }

    let data = sunset_data::fetch_sunrise_sunset(lat, long).unwrap();
    println!("{:#?}", data);

    // Wait for 1 minute
    thread::sleep(time::Duration::from_secs(1));
}

fn set_temperature(temperature: i32) -> Result<(), String> {
    Command::new("hyprctl")
        .args(["hyprsunset", "temperature", &temperature.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    Ok(())
}

fn get_current_temperature() -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["hyprsunset", "temperature"])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Convert output to string
    let stdout =
        str::from_utf8(&output.stdout).map_err(|e| format!("Invalid UTF-8 in output: {}", e))?;

    // Parse the string to i32
    stdout
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("Failed to parse temperature: '{}'", stdout))
}

fn toggle_temperature(temp_day: i32, temp_night: i32) -> Result<(), String> {
    let current_temp = get_current_temperature()?;

    let new_temp = if current_temp == temp_day {
        temp_night
    } else {
        temp_day
    };

    println!(
        "Current temperature: {}K, setting to: {}K",
        current_temp, new_temp
    );

    set_temperature(new_temp)
}
