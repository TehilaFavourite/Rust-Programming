use std::io;

fn main() {
    println!("This is a temperature converter");
    println!("Celsius to Farenheit");
    println!("Fahrenheit to Celsuis");
    println!("Please select an option, (1 or 2): ");
    
    let mut user_choice = String::new(); // create a mutable string to store the input
    
    // Read a line from standard input and store it in user_choice
    io::stdin().read_line(&mut user_choice)
        .expect("Failed to read line"); // handle potential errors
    
    // parse the string into a u32
    let user_choice: u32 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number");
            return;
        }
    };
    if user_choice == 1 {
        println!("You want to convert Celsius to Farenheit");
        celsius_to_fahrenheit();
    } else if user_choice == 2 {
        println!("You want to convert Fahrenheit to Celsuis");
        fahrenheit_to_celsius();
    } else {
        println!("Invalid input, Please enter a valid input");
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter Temperature in Celsius");
    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp)
        .expect("Failed to read line"); // handle potential errors
    
    // parse the string into a u32
    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number");
            return;
        }
    };
    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2} in Celcius is {:.2} in Fahrenheit", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter Temperature in Fahrenheit"); 
    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp)
        .expect("Failed to read line"); // handle potential errors
    
    // parse the string into a u32
    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number");
            return;
        }
    };
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2} in Fahrenheit is {:.2} in Fahrenheit", temp, celsius);
}

