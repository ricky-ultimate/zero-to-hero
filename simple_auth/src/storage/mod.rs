use crate::models::user::User;
use serde_json;
use std::fs;
use std::io;

const USERS_FILE: &str = "users.json";

pub fn load_users() -> Vec<User> {
    match fs::read_to_string(USERS_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

pub fn save_users(users: &Vec<User>) -> io::Result<()> {
    match serde_json::to_string(&users) {
        Ok(json) => fs::write(USERS_FILE, json),
        Err(e) => {
            eprintln!("Failed to serialize users: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}
