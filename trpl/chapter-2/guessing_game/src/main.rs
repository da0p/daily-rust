/*
 * Bring the io input/output library into scope. The io library comes from the
 * standard library.
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // A mutable variable
        let mut guess = String::new();

        // read a line into the mutable 'guess' variable, if failed, print the line
        // in expect
        // &mut means reference to the mutable variable
        // read_line will return a Result. An expect will take the return value
        // Ok is holding and return just that value
        // Otherwise, if there is an error, expect will cause the program to crash
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim method on String instance willl eliminate any whitespace at the
        // beginning and end
        // parse method on String converts a string into another type and we need
        // to tell Rust the exact number type we want
        // if parse() suceeds, it will return the Ok variant of Result, and expect
        // will return the number that we want from the Ok value (unwrapping)
        // Otherwise, the program will crash
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // A match expression is made up of arms. An arm consists of a pattern to
        // match against, and the code should be run if the value given to match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
