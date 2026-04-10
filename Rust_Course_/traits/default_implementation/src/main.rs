use std::collections::HashMap;

trait Accomodation {
    fn get_description(&self) -> String {
        String::from("This is a place to stay.")
    }

    fn book(&mut self, name: &str, nights: u32); 
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accomodation for Hotel {
    // fn get_description(&self) -> String {
    //     format!("{} is a hotel with {} reservations.", self.name, self.reservations.len())
    // }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests:Vec<(String, u32)>
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accomodation for AirBnB {
    fn get_description(&self) -> String {
        format!("{} is an AirBnB hosted by {} with {} guests.", self.host, self.host, self.guests.len())
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn main() {
    let mut hotel = Hotel::new("The Grand Tee");
    println!("{}", hotel.get_description());
    hotel.book("Alice", 3);

    let mut airbnb = AirBnB::new("Bob");
    println!("{}", airbnb.get_description());
    airbnb.book("Charlie", 2);
    println!("{:#?}", airbnb);

}


