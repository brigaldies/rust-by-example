// CLI command matching using enums.
//
// Source: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=85e642ecf9b53834d025c814dfdc4055

use std::io;
use std::io::Write;

// Declare valid commands with an enum
enum Command {
    Add(String),
    Edit(String),
    Exit(),
    Unknown {
        command: String,
        args: String,
    },
}

// This allows use to change (String, String) into Command enum.
impl From<(String, String)> for Command {
    fn from((command, args): (String, String)) -> Self {
        // We use &str from command
        match command.as_str() {
            "add" => Self::Add(args),
            "edit" => Self::Edit(args),
            "exit" => Self::Exit(),
            _ => Self::Unknown {
                command,
                args,
            }
        }
    }
}

// Parse a command
fn parse_command(command: Command) -> bool {
    let mut exit = false;
    // Now we can match the Command enum
    match command {
        Command::Add(args) => println!("Add command {:?}", args),
        Command::Edit(args) => println!("Edit command {:?}", args),
        Command::Exit() => {
            println!("Exit command");
            exit = true;
        }
        Command::Unknown { command, args } => println!("Something else command {:?}, word {:?}", command, args),
    }

    return exit;
}

pub(crate) fn cli_command_processing() {
    loop {
        let mut user_input = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap(); // unwrap implicitly handles any error
        io::stdin()
            .read_line(&mut user_input)
            .expect("An error occurred while reading thr stdio");

        let mut input_split = user_input.split_whitespace();

        let cmd = String::from(input_split.next().unwrap());
        let mut args: String = "".to_owned();
        for arg in input_split {
            args.push_str(arg);
        }
        let command = Command::from((cmd, args));
        if parse_command(command) {
            break;
        }
    }
}