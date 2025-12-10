use std::io;

fn main() {
    println!("welcome to the basic calculator.");
    println!("Here, you can add, subtract, multiply, and divide");
    println!("available operations are '+', '-', '*', '/' ");
    println!("please enter th operation like this: e.g., 5 + 5");
    
    let mut user_input = String::new();
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
        
    //  1. Split the input string into parts (e.g., ["10", "+", "5"])
    let parts: Vec<&str> = user_input.trim().split_whitespace().collect();
    
    if parts.len() != 3 {
        println!("invalid input. Please follow the format of the operation");
        return;
    }
    
    // 2. Parse the first number using match
    let num1: f64 = match parts[0].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Error: Invalid first number.");
            return;
        }
    };

    // 3. Get the operator
    let operator = parts[1];
    
    // 4. Parse the second number using match
    let num2: f64 = match parts[2].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Error: Invalid second number.");
            return;
        }
    };
    
    // 5. Perform the calculation using another match expression on the operator
    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Error: Unknown operator '{}'. Use +, -, *, or /", operator);
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Cannot divide by zero.");
        std::process::exit(1);
    }
    a / b
}