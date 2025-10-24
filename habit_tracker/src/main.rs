mod habit;
mod storage;

use habit::Habit;
use std::io::{self, Write};
use storage::{load_habits, save_habits};

fn main() {
    let mut habits: Vec<Habit> = load_habits();

    println!("Welcome to habit tracker!");

    loop {
        print!("Type a command (add/list/done/quit): ");
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

                let new_habit = Habit::new(name);
                habits.push(new_habit);
                save_habits(&habits);
                println!("Habit added and saved!")
            }
            "list" => {
                println!("Your habits: ");
                for habit in &habits {
                    habit.summary();
                }
            }
            "done" => {
                print!("Which habit did you complete today?: ");
                io::stdout().flush().expect("Failed to flish to stdout");

                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("failed to read name");
                let name = name.trim();

                if let Some(habit) = habits.iter_mut().find(|h| h.name == name) {
                    habit.mark_done();
                    save_habits(&habits);
                } else {
                    println!("Habit '{}' not found", name);
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
