mod models;
mod services;
mod storage;

use services::{login, signup};
use std::io::{self, Write};

fn main() {
    println!("🪪 Simple Auth Service");

    loop {
        print!("Command (signup/login/quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        match cmd {
            "signup" => {
                let (username, password) = get_input();
                signup(username.trim(), password.trim());
            }
            "login" => {
                let (username, password) = get_input();
                login(username.trim(), password.trim());
            }
            "quit" => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}

fn get_input() -> (String, String) {
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    (username, password)
}
