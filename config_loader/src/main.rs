mod config;

fn main() {
    let config_path = "test_data/test.env";

    match config::load_config(config_path) {
        Ok(cfg) => {
            if let Some(port) = cfg.port {
                println!("SERVER RUNNING ON PORT: {}", port);
            }

            if cfg.debug.unwrap_or(false) {
                println!("Debug enabled");
            }

            if let Some(ref db) = cfg.database {
                println!("Connecting to database at: {} ", db.url);
            }

            println!("Loaded config: {:#?}", cfg)
        }
        Err(e) => eprintln!("Error loading config: {:?}", e),
    }
}
