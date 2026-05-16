#[derive(PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

// impl PartialEq for Flight {
//     fn eq(&self, other: &Self) -> bool {
//         self.origin == other.origin && self.destination == other.destination && self.time == other.time
//     }
// }

fn main() {
    let a = Flight::new("New York", "Los Angeles", "10:00 AM");
    let b = Flight::new("New York", "Los Angeles", "10:00 AM");
    let c = Flight::new("Chicago", "Miami", "2:00 PM");

    println!("Is a equal to a? {}", a.eq(&a));
    println!("Is a equal to b? {}", a == b);
    println!("Is a equal to c? {}", a == c);
    println!("Are a and b equal? {}", a.eq(&b));
    println!("Are a and c equal? {}", a.eq(&c));
    println!("Are a and b not equal? {}", a.ne(&b));
    println!("Are a and c not equal? {}", a.ne(&c));
}
