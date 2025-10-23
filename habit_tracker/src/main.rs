use ::std::io::{self, Write};
use:: std::fs;
use::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Habit {
    name: String,
    dates_done: Vec<String>,
}
fn main() {
    let mut habits: Vec<Habit> = load_habits();

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
                save_habits(&habits);
                println!("Habit added and saved!")
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

fn save_habits(habits: &Vec<Habit>) {
    let data = serde_json::to_string_pretty(habits).expect("Failed to serialize habits");
    fs::write("habits.json", data).expect("Unable to write to file");
}

fn load_habits() -> Vec<Habit> {
    let data = fs::read_to_string("habits.json");
    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}
