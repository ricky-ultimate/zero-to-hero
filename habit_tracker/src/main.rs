use::std::io::{self, Write};

fn main() {
    println!("Welcome to habit tracker!");
    print!("Type a command (add/list/quit): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read commad!");

    println!("You typed: {}", command.trim());
}
