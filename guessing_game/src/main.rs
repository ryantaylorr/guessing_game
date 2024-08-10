use std::io::{self, Read};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello User,");
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

loop{
    loop{
        let mut name = String::new();
        let mut guess = String::new();
        let mut answer = String::new();
        let mut quest = String::new();

        println!("\nWhat is your name?");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        println!("What is your quest?");
            io::stdin()
            .read_line(&mut quest)
            .expect("Failed to read line");
        println!("What is the airspeed velocity of an unladen swallow?\nJust Kidding, we're playing a guessing game.\nwhat is your guess?");
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("\nThe Secret Number was: {secret_number}\nLet's Play Again.\n-------------------------------");
                
                break;
                }
            }
        }
    }
}
