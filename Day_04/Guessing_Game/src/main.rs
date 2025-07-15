use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I'm thinking a number between 1 and 100. Can you guess it?");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid input!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid number!!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. Try again!"),
            Ordering::Greater => println!("Too big. Try again!"),
            Ordering::Equal => {
                println!("Congratulations! Correct number!!");
                return;
            }
        }
    }
    
}