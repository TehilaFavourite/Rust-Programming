#[derive(PartialEq)]
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
    let flight1 = Flight::new("New York", "Los Angeles", "10:00 AM");
    let flight2 = Flight::new("New York", "Los Angeles", "10:00 AM");
    let flight3 = Flight::new("Chicago", "Miami", "2:00 PM");

    println!("Is flight1 equal to flight2? {}", flight1 == flight2);
    println!("Is flight1 equal to flight3? {}", flight1 == flight3);
    println!("{}", flight1.eq(&flight2));
    println!("{}", flight1.ne(&flight3));
}
