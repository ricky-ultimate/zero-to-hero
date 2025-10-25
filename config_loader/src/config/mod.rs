use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(String),
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(ConfigError::Io)
}

pub fn parse_env(content: &str) -> Result<HashMap<String, String>, ConfigError> {
    let mut config = HashMap::new();

    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        match line.split_once("=") {
            Some((key, value)) => {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                if config.contains_key(&key) {
                    println!(
                        "Warning: duplicate key '{}' found (line {})",
                        key,
                        line_no + 1
                    );
                }

                config.insert(key, value);
            }

            None => {
                return Err(ConfigError::Parse(format!(
                    "Invalid line {}: '{}'",
                    line_no + 1,
                    line
                )));
            }
        }
    }
    Ok(config)
}
