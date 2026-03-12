use std::fs::File;
use std::io::{stdin, Read};
use std::process;

fn main() {

    println!("Please enter the name of the file you like to read: ");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("something went wrong reading the user input. The error was {error:?}");
        process::exit(1);
    }


    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("something went wrong opening the file. The error was {error:?}");
            process::exit(1);
        }
    };

    let mut file_contents = String::new();

    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprintln!("something went wrong reading the file content. The error was {error:?}");
        process::exit(1);
    }

    println!("opened file handle: {file_contents}");
}