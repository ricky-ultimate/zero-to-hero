use ::std::io::{self, Write};

fn main() {
    println!("Welcome to habit tracker!");

    loop {
        print!("Type a command (add/list/quit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read commad!");

        let command = command.trim();

        match command {
            "add" => println!("You chose to add a habit!"),
            "list" => println!("You chose to list your habits"),
            "quit" => {
                println!("You chose to exit the program");
                break;
            }
            _ => println!("Unidentified command, please try again."),
        }
    }
}
