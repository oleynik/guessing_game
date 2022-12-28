use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Incorrect input!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big..."),
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Exit")
}
