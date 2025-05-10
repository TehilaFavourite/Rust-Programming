use std::io;

fn main() {
    println!("Unit Converter");
    println!("Choose an option:");
    println!("1. Convert Celsius to Fahrenheit");
    println!("2. Convert Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    match choice.trim() {
        "1" => {
            println!("Enter temperature in Celsius:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read temperature");
            let celsius: f64 = input.trim().parse().expect("Invalid input");

            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
        }

        "2" => {
            println!("Enter temperature in Fahrenheit:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read temperature");
            let fahrenheit: f64 = input.trim().parse().expect("Invalid input");

            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("Temperature in Celsius: {:.2}", celsius);
        }

        _ => {
            println!("Invalid choice.");
        }
    }

}
// Step 5: Conversion functions
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
