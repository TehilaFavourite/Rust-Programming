use traits::{book_for_one_night, mix_and_match, Accomodation, Description, Hotel, AirBnB};
fn main() {
    let mut hotel1 = Hotel::new(String::from("The Grand Tee"));
    println!("{}", hotel1.summarize());
    hotel1.book("Charlie", 3);

    let mut airbnb1 = AirBnB::new("Alice");
    println!("{}", airbnb1.get_description());
    book_for_one_night(&mut hotel1, "Dave");

    mix_and_match(&mut hotel1, &mut airbnb1, "Eve");
}
