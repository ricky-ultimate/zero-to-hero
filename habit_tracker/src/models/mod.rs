use chrono::Local;
use serde::{Deserialize, Serialize};

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

    pub fn mark_done(&mut self) -> String {
        let today = Local::now().format("%y-%m-%d").to_string();
        if !self.dates_done.contains(&today) {
            self.dates_done.push(today.clone());
            println!("Marked '{}' as done for '{}'", self.name, today)
        } else {
            println!("'{}' was already marked done for '{}'", self.name, today);
        }

        today
    }

    pub fn summary(&self) {
        println!("- {} ({} times done)", self.name, self.dates_done.len());
    }
}
