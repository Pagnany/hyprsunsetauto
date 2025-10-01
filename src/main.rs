use chrono::Local;
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

    thread::sleep(time::Duration::from_secs(10));

    // Setup
    let mut just_started = true;
    let mut temperature_current = get_current_temperature().unwrap_or(temperature_day);
    let mut date = Local::now().date_naive();

    let (mut time_sunrise, mut time_sunset) =
        match sunset_data::get_sunrise_sunset_for_today(lat, long) {
            Ok((sunrise, sunset)) => (sunrise, sunset),
            Err(e) => {
                eprintln!("Error fetching sunrise/sunset data: {}", e);
                panic!();
            }
        };

    loop {
        // if Date has changed, fetch new sunrise/sunset times
        let current_date = Local::now().date_naive();
        if current_date != date {
            println!("Date changed from {} to {}", date, current_date);
            date = current_date;

            (time_sunrise, time_sunset) = match sunset_data::get_sunrise_sunset_for_today(lat, long)
            {
                Ok((sunrise, sunset)) => (sunrise, sunset),
                Err(e) => {
                    eprintln!(
                        "Error fetching sunrise/sunset data: {}\n Retry in 60 Sec",
                        e
                    );
                    thread::sleep(time::Duration::from_secs(60));
                    // Try again on next iteration
                    continue;
                }
            };
        }

        let time_current = Local::now().time();
        println!("Current local time: {}", time_current);
        println!("Sunrise: {}, Sunset: {}", time_sunrise, time_sunset);

        // Check if we have to change the temperature
        if time_current < time_sunrise || time_current > time_sunset {
            println!("It's night: {}K", temperature_current);
            if temperature_current != temperature_night {
                if just_started {
                    println!(
                        "Just started, setting temperature immediately to {}K",
                        temperature_night
                    );
                    if let Err(e) = set_temperature(temperature_night) {
                        eprintln!("Error setting temperature: {}", e);
                    }
                } else {
                    println!("Setting temperature to {}K", temperature_night);
                    transition(temperature_current, temperature_night, 10);
                }
                temperature_current = temperature_night;
            }
        } else {
            println!("It's day: {}K", temperature_current);
            if temperature_current != temperature_day {
                println!("Setting temperature to {}K", temperature_day);
                transition(temperature_current, temperature_day, 10);
                temperature_current = temperature_day;
            }
        }

        just_started = false;
        thread::sleep(time::Duration::from_secs(60));
    }
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

/// duration in minutes
fn transition(temperature_from: i32, temperature_to: i32, duration: i32) {
    // seconds
    let update_interval_sec = 10;

    let temp_diff = temperature_to - temperature_from;

    let steps = duration * 60 / update_interval_sec;
    let step_size = temp_diff as f32 / steps as f32;
    for i in 0..steps {
        let current_temp = (temperature_from as f32 + step_size * i as f32) as i32;
        if let Err(e) = set_temperature(current_temp) {
            eprintln!("Error during transition: {}", e);
            break;
        }
        thread::sleep(time::Duration::from_secs(update_interval_sec as u64));
    }

    // To be shure that we are a the right end temperature
    if let Err(e) = set_temperature(temperature_to) {
        eprintln!("Error during transition: {}", e);
    }
}
