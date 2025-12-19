use std::fs;
use std::io;
use crate::habit::Habit;  // ← Import from our module!
use serde_json;

pub fn save_habits(habits: &[Habit]) -> Result<(), io::Error> {
    let json_payload = serde_json::to_string_pretty(habits).map_err(|e|io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("habits.json", json_payload)?;
    Ok(())
}

pub fn load_habits() -> Result<Vec<Habit>, io::Error> {
    let data = fs::read_to_string("habits.json")?;
    serde_json::from_str(&data).map_err(|e|io::Error::new(io::ErrorKind::Other, e))
}