use std::fs;
// use std::process;
use std::io;

// fn main() {
//     let directory = fs::read_dir("./").unwrap_or_else(|error| {
//         eprintln!("Could not read directory: {error}");
//         process::exit(1);
//     });
    
//     for entry_result in directory {
//         match entry_result {
//             Ok(entry) => println!("{:?}", entry.path()),
//             Err(error) => {
//                 eprintln!("Could not entry: {error}");
//             }
//         }
//     }
// }

// fn main() -> io::Result<()> {
//     for entry_result in fs::read_dir("./")? {
//         match entry_result {
//             Ok(entry) => println!("{:?}", entry.path()),
//             Err(error) => {
//                 eprintln!("Could not entry: {error}");
//             }
//         }
//     }
//     Ok(())
// }

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("./")? {
        let entry = entry_result?;
        // println!("{:?}", entry.path());
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name);
        // if metadata.is_file() {
        if metadata?.is_file() {
            println!("{entry_name:?}\n-------");
            let contents = fs::read_to_string(&entry_name)?;
            println!("{contents}");
        }
    }
    Ok(())
}