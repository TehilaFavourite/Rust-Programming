use std::fs::File;
use std::process;

fn main() {
    // try to open `story.txt` in the same directory as the binary
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };

    // `file` is used below – at least print its debug representation
    println!("opened file handle: {file:?}");
}
