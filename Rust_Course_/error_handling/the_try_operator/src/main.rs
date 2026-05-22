use std::fs::File;
use std::io::{self, Read, stdin};
use std::process;

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(content) => println!("opened file handle: {content}"),
        Err(error) => {
            eprintln!("something went wrong. The error was {error:?}");
        }
    }
}

fn read_file() -> Result<String, std::io::Error> {
    println!("Please enter the name of the file you like to read: ");

    let mut input = String::new();

    stdin().read_line(&mut input)?;
    
    let mut file_contents = String::new();
    let file = File::open(input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}
