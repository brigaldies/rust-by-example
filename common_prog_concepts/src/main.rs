use std::io;
use std::io::Write;

mod variables;
mod match_deconstructing;

// enum CommandName {
//     QUIT(String),
//     EXIT(String),
// }
//
// const QUIT: CommandName = CommandName::QUIT(String::from("quit"));
// const EXIT: CommandName = CommandName::QUIT(String::from("exit"));

fn main() {
    menu();
}

fn menu() {
    loop {
        print_menu();
        let mut selection = String::new();
        print!("Enter your example number: ");
        io::stdout().flush().unwrap(); // unwrap implicitly handles any error
        io::stdin()
            .read_line(&mut selection)
            .expect("An error occurred while reading your guessed number.");

        let selection_cleaned = selection.trim().to_lowercase();

        let selection = match selection_cleaned.as_str() {
            "quit" => break,
            "exit" => break,
            _ => {}
        };

        let selection: u32 = match selection_cleaned.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match selection {
            1 => variables::variables_are_immutable(),
            2 => variables::variables_can_be_mutable(),
            3 => match_deconstructing::match_deconstructing_pointers_and_references(),
            _ => {
                println!("Please enter an example menu number.");
                continue;
            }
        }
    }
}

fn print_menu() {
    println!("\n-------------------------------------");
    let mut i = 1;
    println!("{i}: Variables are immutable");
    i += 1;
    println!("{i}: Variables can be mutable");
    i += 1;
    println!("{i}: Match deconstructing pointers and references");

    println!("\nType a menu number between 1 and {i}")
}