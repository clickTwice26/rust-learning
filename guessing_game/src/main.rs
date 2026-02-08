use std::io;
use rand::{Rng, rngs::ThreadRng};
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    println!("Guess a number: ");
    
    let secret_number : i32 = rand::rng().random_range(1..=100);
    let mut guess : String = String::new();    
    io::stdin().read_line(&mut guess).expect("Failed to read user input");
    // println!("You guessed {guess}");
    let guess : i32 = guess.trim().parse().expect("Failed to parse!");
    // println!("You guessed {guess}");
    
    // println!("Thanks");
    // println!("{guess}");
    // println!("Ok Bye");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{guess} is less"),
        Ordering::Equal => println!("You won!!"),
        Ordering::Greater => println!("{guess} is greater")
    };
}
