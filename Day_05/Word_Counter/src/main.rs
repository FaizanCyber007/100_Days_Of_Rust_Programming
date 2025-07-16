use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <filepath>");
        return;
    }

    let file_path = &args[1];

    let mut file: File = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Problem reading file. Try again!");
            return;
        }
    };

    let mut content = String::new();

    if let Err(err) = file.read_to_string(&mut content) {
        println!("Error opening file {}!", err);
        return;
    }

    let word_count = count_words(&content);

    println!("Total number of words in this file is {}", word_count);
}

fn count_words(a: &str) -> usize {
    a.split_whitespace().count()
}