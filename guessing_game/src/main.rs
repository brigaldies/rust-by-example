use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("Hello, this is a guessing game!");
    println!("Enter a number between 1 and 100.");

    // Generate a random secret
    let secret = rand::thread_rng().gen_range(1..=100);
    // println!("Secret: {secret}");

    loop {
        // Collect the player's guess
        let mut guess = String::new();
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // unwrap implicitly handles any error
        io::stdin()
            .read_line(&mut guess)
            .expect("An error occurred while reading your guessed number.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 1 and 100");
                continue;
            },
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            },
        };
    }
}
