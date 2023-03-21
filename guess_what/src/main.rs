use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=10);

    println!("Guess the number!");

    loop {
        println!("Please input your guess:");

        let mut guess_fmt: (String, u32) = (String::new(), 0);
        io::stdin()
            .read_line(&mut guess_fmt.0)
            .expect("Failed to read line");
        guess_fmt.1 = match guess_fmt.0.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            println!("Please type a valid number!");
                continue;
            }
        };

        println!("Your guessed: {}", guess_fmt.1);

        match guess_fmt.1.cmp(&secret_number) {
            Ordering::Less => println!("Too small, retry! :)"),
            Ordering::Greater => println!("Too big, retry! :)"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number was {}", secret_number);
}
