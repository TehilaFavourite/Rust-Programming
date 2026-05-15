use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

#[allow(dead_code)]
enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let apple_name = match self {
            AppleType::RedDelicious => "Red Delicious",
            AppleType::GrannySmith => "Granny Smith",
        };
        write!(f, "{}", apple_name)
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let apple_name = match self {
            AppleType::RedDelicious => "Red Delicious",
            AppleType::GrannySmith => "Granny Smith",
        };
        write!(f, "{}", apple_name)
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("The apple has been dropped and the file has been removed."),
            Err(e) => println!("The apple has been dropped but there was an error removing the file: {}", e),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "The {} 🍏 costs ${:.2}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("** Apple **").field("kind", &self.kind).field("price", &self.price).finish()
    }
}

fn main() {
    fs::write("apple.txt", "").expect("Failed to create apple.txt");
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 0.99,
    };
    println!("{:?}", lunch_snack);
    println!("{}", lunch_snack);
}
