// the_match_keyword_with_option_enum
fn main() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("bass"),
    ];
    let bass: Option<&String> = musical_instrument.get(2);
    println!("{:?}", bass);
    
    play(bass);
    
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    };
}
