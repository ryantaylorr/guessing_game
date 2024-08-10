use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello User,");
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

loop{
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
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
                println!("The Secret Number was: {secret_number}");
                println!("Let's Play Again.");
                
                break;
                }
            }
        }
    }
}
