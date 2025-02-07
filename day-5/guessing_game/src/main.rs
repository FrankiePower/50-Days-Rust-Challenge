// Implementing a guessing game. The program will generate a random number between 1 and 100, and then challenge the player to guess the number. After each guess, the program will tell the player if the guess was too high or too low.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // creates a mutable variable that is currently bound to a new, empty instance of a String.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_count += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you: {} guesses", guess_count);
                break;
            }
        }
    }
}
