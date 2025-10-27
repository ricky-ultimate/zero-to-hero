use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(String),
    Toml(toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub debug: Option<bool>,
    pub database: Option<DatabaseConfig>,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref();

    match path.extension().and_then(|e| e.to_str()) {
        Some("toml") => {
            let content = fs::read_to_string(path).map_err(ConfigError::Io)?;
            let structured: Config = toml::from_str(&content).map_err(ConfigError::Toml)?;
            Ok(structured)
        }

        Some("env") => {
            let content = fs::read_to_string(path).map_err(ConfigError::Io)?;
            let env_map = parse_env(&content)?;

            Ok(Config {
                port: env_map.get("PORT").and_then(|p| p.parse().ok()),
                debug: env_map.get("DEBUG").and_then(|d| d.parse().ok()),
                database: env_map
                    .get("URL")
                    .map(|url| DatabaseConfig { url: url.clone() }),
            })
        }

        None | _  => Err(ConfigError::Parse(format!(
            "unsupported config format: {:?}",
            path
        ))),
    }
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
