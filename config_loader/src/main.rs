mod config;

fn main() {
    let path = "test_data/.env";

    match config::read_file(path) {
        Ok(content) => match config::parse_env(&content) {
            Ok(config_map) => {
                for (key, value) in &config_map{
                    println!("{} : {}", key, value)
                }
            }

            Err(e) => eprintln!("Parse error: {:?}", e),
        },
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
