use std::fs;
use crate::Calc;

pub fn save_to_file(calc: &Calc, path: &str) -> Result<(), std::io::Error> {
    fs::write(path, "Hello World")?;
    return Ok(());
}

pub fn load_from_file(path: &str) -> Result<Calc, std::io::Error> {
    println!{"Loaded Calcxulator from File"}
    let file_contents: String = String::from_utf8(fs::read(path)?).expect("Could not parse file contents to a string...");

    let mut calc = Calc::new(15);
    Ok(calc)
}
