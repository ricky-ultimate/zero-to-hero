use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn parse_env(content: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if let Some((key, value)) = line.split_once("=") {
            config.insert(key.trim().to_string(), value.trim().to_string());
        } else {
            println!("Warning: Invalid line '{}'", line);
        }
    }
    config
}
