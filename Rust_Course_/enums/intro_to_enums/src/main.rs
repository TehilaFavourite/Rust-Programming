#[derive(Debug)]

enum CardSuits {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuits,
}

fn main() {
    let first_card = CardSuits::Hearts;
    let mut second_card = CardSuits::Diamonds;
    println!("This is the second card: {:?}", second_card);
    
    let card_suits = [CardSuits::Hearts, CardSuits::Clubs];
    println!("This is the card_suits: {:?}", card_suits);
}