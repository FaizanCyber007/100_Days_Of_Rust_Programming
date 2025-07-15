use std::io;

fn main() {
    println!("Simple Calculator!!");
    println!("Available Operations: +, -, *, /");
    println!("Enter your expression (e.g. 5 + 3 (including spaces)): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input!");
    
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid input. Please recheck the format");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number!");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number!");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("The answer of the operation is: {:.2}", result);
}


fn add(n1: f64, n2: f64) -> f64 {
    n1+n2
}

fn subtract(n1: f64, n2: f64) -> f64 {
    n1-n2
}

fn multiply(n1: f64, n2: f64) -> f64 {
    n1*n2
}

fn divide(n1: f64, n2: f64) -> f64 {
    if n2 == 0.0 {
        println!("Division by 0 is prohibited!!");
        std::process::exit(1);
    }

    n1/n2
}