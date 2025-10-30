#[derive(Debug)]

enum CardSuits {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

fn main() {
    let first_card = CardSuits::Hearts;
    let mut second_card = CardSuits::Diamonds;
    println!("This is the second card: {:?}", second_card);
}