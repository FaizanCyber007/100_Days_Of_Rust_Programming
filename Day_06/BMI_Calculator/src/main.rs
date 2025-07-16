use std::{fs::read, io};

fn main() {
    println!("BMI Calculator!!");

    println!("Enter your weight in kilogram!!");
    let weight = match get_input_in_f64() {
        Some(value) => value,
        None => {
            println!("Invalid Input. Please recheck it!!");
            return;
        }
    };

    println!("Enter the height in meter!!");
    let height = match get_input_in_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input. Please recheck!");
            return;
        }
    };

    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {}", bmi);

    let category = classify_bmi(bmi);
    println!("BMI category: {}", category);
}

fn get_input_in_f64() -> Option<f64> {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Input not found!!");

    match input.trim().parse::<f64>() {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

fn calculate_bmi(a: f64, b: f64) -> f64 {
    a / (b * b)
}

fn classify_bmi(a: f64) -> &'static str {
    if a < 18.5 {
        "Underweight"
    } else if a >= 18.5 && a <= 24.9 {
        "Normal Weight"
    } else if a >= 25.0 && a <= 29.9 {
        "Overweight!"
    } else {
        "Obesity"
    }
}