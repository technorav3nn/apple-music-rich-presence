use std::process::Command;

use serde::de::DeserializeOwned;

pub fn run_osascript_js<T>(scr: &str) -> Result<T, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let command = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(scr)
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&command.stdout);

    match serde_json::from_str::<T>(&output) {
        Ok(ser_output) => {
            println!("ser_output");
            Ok(ser_output)
        }
        Err(e) => Err(e.to_string()),
    }
}
