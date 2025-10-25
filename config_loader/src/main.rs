mod config;

fn main() {
    let path = "test_data/.env";

    match config::read_file(path) {
        Ok(content) => {
            let config_map = config::parse_env(&content);
            for (key, value) in config_map {
                println!("{} : {}", key, value)
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
