use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Serialize, Deserialize)]
pub struct Habit {
    pub name: String,
    pub dates_done: Vec<String>,
}

impl Habit {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dates_done: Vec::new(),
        }
    }

    pub fn mark_done(&mut self) {
        let today = Local::now().format("%y-%m-%d").to_string();
        self.dates_done.push(today.clone());
        println!("Marked '{}' as done for '{}'", self.name, today)
    }

    pub fn summary(&self) {
        println!("- {} ({} times done)", self.name, self.dates_done.len());
    }
}
