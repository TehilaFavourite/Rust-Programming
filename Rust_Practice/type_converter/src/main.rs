// Step 2: Set Up Project
// This goes into src/main.rs

use std::io;

fn main() {
    println!("ASCII Converter");
    println!("Choose an option:");
    println!("1. Convert character to ASCII");
    println!("2. Convert ASCII value to character");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            println!("Enter a single character:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read character");
            let input = input.trim();

            if let Some(c) = input.chars().next() {
                let ascii = char_to_ascii(c);
                println!("ASCII value of '{}' is: {}", c, ascii);
            } else {
                println!("Invalid input: no character entered.");
            }
        }

        "2" => {
            println!("Enter an ASCII value (0 - 127):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read number");

            match input.trim().parse::<u8>() {
                Ok(n) if n <= 127 => {
                    let character = ascii_to_char(n);
                    println!("Character for ASCII value {} is: '{}'", n, character);
                }
                _ => {
                    println!("Invalid input: enter a number between 0 and 127.");
                }
            }
        }

        _ => {
            println!("Invalid choice.");
        }
    }
}

// Step 5: Conversion functions

fn char_to_ascii(c: char) -> u8 {
    c as u8
}

fn ascii_to_char(n: u8) -> char {
    n as char
}