use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guess the number! Please make your guess.");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Line read failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Naughty naughty! That's not valid. Numbers only please.");continue},
        };
        println!("Your guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Your guess was too small. Try again."),
            Ordering::Greater => println!("Your guess was too large. Try again."),
            Ordering::Equal => {
                println!("You won! Good job.");
                break;
            },
        }
    }

   
}
