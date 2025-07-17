use std::io;

fn main() {
    println!("Palindrome Checker!!");
    println!("Please enter a string!");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Could read input");

    let mut cleaned_input = cleaned_input(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a non-empty string!");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("{} is a palindrome", cleaned_input);
    } else {
        println!("{} is not a palindrome", cleaned_input);
    }
}

fn cleaned_input(input: &str) -> String {
    input
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_lowercase().to_string())
    .collect()
}

fn is_palindrome(input: &str) -> bool {
    input == input.chars().rev().collect::<String>()
}