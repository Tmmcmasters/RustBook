use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}"); // This line prints the secret number to the console if you want to test it.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // let x = 5;
    // let y = 10;

    // This line uses string interpolation to print out the values of x and y+2.
    // The {} is a placeholder where the value will be inserted.
    // The ,y+2 after the string is the value that gets inserted.
    // println!("x = {x} and y + 2 = {}", y + 2);
}
