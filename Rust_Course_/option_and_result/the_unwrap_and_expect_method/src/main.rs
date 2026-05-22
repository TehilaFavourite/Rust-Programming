fn main() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("bass"),
    ];
    let bass: Option<&String> = musical_instrument.get(2);
    println!("{:?}", bass);

    let invalid = musical_instrument.get(100);
    println!("{:?}", invalid);

    let valid_instrument = bass.unwrap();
    println!("The valid instrument is: {}", valid_instrument);

    // this will panic if invalid is None
    let invalid_instrument = invalid.expect("Instrument not found");
    println!("The invalid instrument is: {}", invalid_instrument);
}
