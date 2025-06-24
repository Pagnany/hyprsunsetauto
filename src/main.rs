use std::process::Command;
use std::{thread, time};

pub mod sunset_data;

fn main() {
    if let Err(e) = toggle_temperature() {
        eprintln!("Error: {}", e);
    }
    // let data = sunset_data::fetch_sunrise_sunset(52.311575, 7.433870).unwrap();
    // println!("{:#?}", data);

    // Wait for 1 minute
    thread::sleep(time::Duration::from_secs(1));
}

fn set_temperature(temperature: i32) -> Result<(), String> {
    let output = Command::new("hyprctl")
        .args(["hyprsunset", "temperature", &temperature.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    // Print stdout to the console
    if !output.stdout.is_empty() {
        print!(
            "{}",
            str::from_utf8(&output.stdout)
                .map_err(|e| format!("Invalid UTF-8 in output: {}", e))?
        );
    }

    // Print stderr if any
    if !output.stderr.is_empty() {
        eprint!(
            "{}",
            str::from_utf8(&output.stderr)
                .map_err(|e| format!("Invalid UTF-8 in error output: {}", e))?
        );
    }

    Ok(())
}

fn get_current_temperature() -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["hyprsunset", "temperature"])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        // Convert output to string
        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Invalid UTF-8 in output: {}", e))?;

        // Parse the string to i32
        stdout
            .trim()
            .parse::<i32>()
            .map_err(|_| format!("Failed to parse temperature: '{}'", stdout))
    } else {
        // If the command failed, return the error message
        let stderr = str::from_utf8(&output.stderr)
            .map_err(|e| format!("Invalid UTF-8 in error output: {}", e))?;
        Err(format!("Command failed: {}", stderr))
    }
}

fn toggle_temperature() -> Result<(), String> {
    // Get the current temperature
    let current_temp = get_current_temperature()?;

    // Choose the new temperature based on the current one
    let new_temp = if current_temp == 3500 { 6500 } else { 3500 };

    println!(
        "Current temperature: {}K, setting to: {}K",
        current_temp, new_temp
    );

    // Set the new temperature
    set_temperature(new_temp)
}
