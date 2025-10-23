use ::std::io::{self, Write};

struct Habit {
    name: String,
    dates_done: Vec<String>,
}
fn main() {
    let mut habits: Vec<Habit> = Vec::new();

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
            "add" => {
                print!("Enter habit name: ");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("failed to read name");
                let name = name.trim().to_string();

                let new_habit = Habit {
                    name,
                    dates_done: Vec::new(),
                };

                habits.push(new_habit);
                println!("Habit added!")
            }
            "list" => {
                println!("Your habits: ");
                for habit in &habits {
                    println!(" - {} ", habit.name)
                }
            }
            "quit" => {
                println!("You chose to exit the program");
                break;
            }
            _ => println!("Unidentified command, please try again."),
        }
    }
}
