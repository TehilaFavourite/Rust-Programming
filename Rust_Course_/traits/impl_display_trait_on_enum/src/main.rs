use std::fmt::{Display, Formatter, Result};

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

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "The {} 🍏 costs ${:.2}", self.kind, self.price)
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 0.99,
    };
    println!("{}" , lunch_snack);
}
