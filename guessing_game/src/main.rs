use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Create an empty string to capture user input
    let mut input = String::new();

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number:");

loop{
   input.clear();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");


let input: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("you are supposed to enter a number");
        continue;
        }
    };
    
    match input.cmp(&secret_number) {
        Ordering::Greater => println!("guess something smaller"),
        Ordering::Less => println!("Guess something bigger"),
        Ordering::Equal => println!("😎 you guessed the correct number"),
    }
}
}
