use traits::{lodging::{Accomodation, Description, Hotel, AirBnB}};
use traits::utils;
fn main() {
    let mut hotel1 = Hotel::new(String::from("The Grand Tee"));
    println!("{}", hotel1.summarize());
    hotel1.book("Charlie", 3);

    let mut airbnb1 = AirBnB::new("Alice");
    println!("{}", airbnb1.get_description());
    utils::book_for_one_night(&mut hotel1, "Dave");

    utils::mix_and_match(&mut hotel1, &mut airbnb1, "Eve");
}
