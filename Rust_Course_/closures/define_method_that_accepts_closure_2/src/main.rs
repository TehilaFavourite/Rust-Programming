#[derive(Debug)]

struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F) where F: FnMut(&Location) {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}
fn main() {
    let locations = [
        Location { name: String::from("Cave"), treasures: 3 },
        Location { name: String::from("Forest"), treasures: 5 },
        Location { name: String::from("Castle"), treasures: 2 },
    ];

    let map = Map { locations: &locations };

    let mut total_treasures = 0;

    map.explore(|location| {
        println!("Exploring {}: {} treasures found!", location.name, location.treasures);
        total_treasures += location.treasures;
    });
    println!("Total treasures found: {}", total_treasures);

    let mut location_names: Vec<String> = Vec::new();
    map.explore(|location| {
        location_names.push(location.name.clone());
    });
    println!("Locations explored: {:?}", location_names);
}
