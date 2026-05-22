use std::fs::File;
use std::io::stdin;
use std::process;

fn main() {

    println!("Please enter the name of the file you like to read: ");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("something went wrong reading the user input. The error was {error:?}");
        process::exit(1);
    }


    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };

    // `file` is used below – at least print its debug representation
    println!("opened file handle: {file:?}");
}
