use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // get number between 1 and 100 is what 1..=100 does
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // for testing and development:
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read_line returns a Result, which is an enum with variants Ok and Err
        // expect is a method that is called on a Result value and crashes the program if the Result is
        // an Err value, and returns the value inside an Ok value
        // guess is updated with the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess is a string, but we want to compare it to a number -> convert to number
        // this is rust convention to shadow previous type with new type for same variable name
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num, // if parse is successful, return the number
            Err(_) => continue, // _ is a catchall value, so this will catch all errors
            // do nothing and continue if invalid input given
        };
        println!("You guessed: {guess}");
        
        // if guess=50 and secret_number=40, guess.cmp(&secret_number) will return Ordering::Greater
        // `match` expressions have "arms" that are evaluated in order (The `Ordering` lines below)
        // ends after first match found
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
