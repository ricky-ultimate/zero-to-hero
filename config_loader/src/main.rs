mod config;
mod services;

fn main() {
    let config_path = "test_data/test.env";

    match config::load_config(config_path) {
        Ok(cfg) => {
            services::start_app(&cfg);
        }
        Err(e) => eprintln!("Error loading config: {:?}", e),
    }
}
