use std::fs;
use serde_yaml;
use crate::Calc;

pub fn save_to_file(calc: &Calc, path: &str) -> Result<(), String> {
    let serialized = match serde_yaml::to_string(calc) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    match fs::write(path, &serialized) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    return Ok(());
}

pub fn load_from_file(path: &str) -> Result<Calc, String> {
    // Get file contents with error handling
    let file_contents: String = match String::from_utf8(
        match fs::read(path) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        }
        ) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    // Rebuild calculator from config file
    let calc = match serde_yaml::from_str(&file_contents) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    Ok(calc)
}
