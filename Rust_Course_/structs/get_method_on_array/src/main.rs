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
}
