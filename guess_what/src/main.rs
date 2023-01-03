use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=10);

    println!("Guess the number!");
    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("Your guessed: {guess}");
    println!("The secret number is {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small, retry! :)"),
        Ordering::Greater => println!("Too big, retry! :)"),
        Ordering::Equal => println!("You win!"),
    }
}
