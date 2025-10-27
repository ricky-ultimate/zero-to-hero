mod config;

fn main() {
    let config_path = "test_data/test.env";

    match config::load_config(config_path) {
        Ok(cfg) => println!("Loaded config: {:#?}", cfg),
        Err(e) => eprintln!("Error loading config: {:?}", e),
    }
}
