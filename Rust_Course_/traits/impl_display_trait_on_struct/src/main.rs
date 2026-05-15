use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "The {} 🍏 costs ${:.2}", self.kind, self.price)
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: String::from("lunch Snack"),
        price: 0.99,
    };
    println!("{}" , lunch_snack);
}
