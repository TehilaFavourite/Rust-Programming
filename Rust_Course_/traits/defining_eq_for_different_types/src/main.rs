struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

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

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination && self.time == other.time
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.origin == other.origin && self.destination == other.destination && self.time == other.time
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.origin == other.origin && self.destination == other.destination && self.time == other.time
    }
}

fn main() {
    let flight = Flight::new("New York", "Los Angeles", "10:00 AM");
    let bus_trip = BusTrip::new("New York", "Los Angeles", "10:00 AM");
    println!("Is the flight equal to the bus trip? {}", flight == bus_trip);
    println!("Does the flight equal the bus trip? {}", flight.eq(&bus_trip));
    println!("Does the bus trip equal the flight? {}", bus_trip.eq(&flight));
    
}
