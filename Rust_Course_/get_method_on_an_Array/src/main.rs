fn main() {
    let musical_instrument = [String::from("Guitar"), String::from("Drums"), String::from("bass"),];
    let first_instrument = musical_instrument.get(0);
    match first_instrument {
        Some(instrument) => println!("The first instrument is: {}", instrument),
        None => println!("There is no instrument at index 0"),
    }
}