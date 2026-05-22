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

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        return Err(error);
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut file_contents = String::new();

    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        return Err(error);
    }

    Ok(file_contents)
}
