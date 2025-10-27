use crate::config::Config;

pub fn start_app(config: &Config) {
    if let Some(port) = config.port {
        println!("Listening on port: {}", port);
    }

    if let Some(ref db) = config.database {
        println!("Connecting to database at: {} ", db.url);
    }

    if config.debug.unwrap_or(false) {
        println!("Debug enabled");
    }
}
