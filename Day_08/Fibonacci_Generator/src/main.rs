use std::{collections::linked_list, io};

fn main() {
    println!("Fibonacci Generator!");
    println!("Enter the number of terms you want to generate sequence upto!!!");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input.");
            return;
        }
    };

    if num_terms == 0 {
        println!("Number of terms must be greater than 0");
        return;
    }

    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence);
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Error reading input");

    match input.trim().parse::<u32>() {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

fn generate_fibonacci(input: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if input >= 1 {
        sequence.push(0);
    } if input >= 2 {
        sequence.push(1);
    }

    for i in 2..input {
        let next = sequence[i as usize-1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence
}