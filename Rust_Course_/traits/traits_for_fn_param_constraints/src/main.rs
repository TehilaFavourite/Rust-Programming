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

    fn summarize(&self) -> String {
        format!("{}: {}.", self.name, self.get_description())
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

fn book_for_one_night(entity: &mut impl Accomodation, guest: &str) {
    entity.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Grand Tee");
    book_for_one_night(&mut hotel, "Alice");
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("Bob");
    book_for_one_night(&mut airbnb, "Charlie");
    println!("{:#?}", airbnb);

}


