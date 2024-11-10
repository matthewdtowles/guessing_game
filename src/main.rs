use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // get number between 1 and 100 is what 1..=100 does
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
