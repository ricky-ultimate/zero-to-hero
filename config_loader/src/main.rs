mod config;

fn main() {
    let path = "test_data/.env";

    match config::read_file(path) {
        Ok(content) => println!("File contents: \n{}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
