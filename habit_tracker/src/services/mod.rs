use std::fs;
use crate::models::Habit;


pub fn load_habits()-> Vec<Habit> {
    let data = fs::read_to_string("habits.json");
    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new()
    }
}

pub fn save_habits(habits: &Vec<Habit>) {
    let data = serde_json::to_string_pretty(habits).expect("Failed to serialize habits");
    fs::write("habits.json", data).expect("Failed to write to file")
}
