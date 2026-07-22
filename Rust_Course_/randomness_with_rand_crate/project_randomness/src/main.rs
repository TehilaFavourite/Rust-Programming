/*
Let's model a card deck!
*/

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

// ---------------------------------------------------------
// "Begin by defining a `Suit` enum with the 4 possible
// suits: Clubs, Spades, Hearts, and Diamonds. Derive the
// Copy, Clone, and Debug traits."
// ---------------------------------------------------------
#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

// ---------------------------------------------------------
// "Define a `Rank` enum with the following ranks: Two,
// Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack,
// Queen, King, Ace, and Joker. Derive the Copy, Clone,
// and Debug traits."
// ---------------------------------------------------------
#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

// ---------------------------------------------------------
// "Define a `Card` struct with a `rank` field set to a Rank
// and a `suit` field set to an Option<Suit>. The reason
// we'll use Option here is because the Joker rank does not
// technically have a suit. Its `suit` will be the None
// variant. Derive the Debug trait."
// ---------------------------------------------------------
#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

// ---------------------------------------------------------
// "Define a `Deck` struct with a single `cards` field set to
// a vector of `Card` structs. Derive the Debug trait."
// ---------------------------------------------------------
#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

// Rust enums don't come with a built-in way to "loop over
// every variant" the way some languages do. The simplest fix
// is a plain array listing them out ourselves. Because Suit
// and Rank derive Copy, we can loop over these arrays by
// value (no borrowing/references needed).
const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];

const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
    // Joker deliberately excluded here -- it gets added
    // separately later via insert_jokers().
];

impl Deck {
    // ---------------------------------------------------------
    // "Define a `new` constructor function on the `Deck`. It
    // should iterate over the 4 possible suits and the 13 main
    // ranks (exclude Joker), create 52 Card instances, populate
    // them into a vector, and then instantiate and return a
    // `Deck` instance with the cards."
    // ---------------------------------------------------------
    fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suit in SUITS {
            for rank in RANKS {
                cards.push(Card {
                    rank,
                    suit: Some(suit),
                });
            }
        }

        Self { cards }
    }

    // ---------------------------------------------------------
    // "Define a `shuffle` method on `Deck` that randomizes the
    // order of the cards."
    // ---------------------------------------------------------
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    // ---------------------------------------------------------
    // "Define a `insert_jokers` method on `Deck`. It should
    // insert 2 Joker cards into the deck at two random index
    // positions. As a reminder, a Joker will have a None suit."
    // ---------------------------------------------------------
    fn insert_jokers(&mut self) {
        let mut rng = thread_rng();

        for _ in 0..2 {
            // 0..=len (inclusive) because inserting AT index
            // len (i.e. appending to the very end) is a valid
            // position -- 0..len would never let a Joker land
            // as the last card.
            let index = rng.gen_range(0..=self.cards.len());
            self.cards.insert(
                index,
                Card {
                    rank: Rank::Joker,
                    suit: None,
                },
            );
        }
    }

    // ---------------------------------------------------------
    // "Define a `delete_random_card` method on `Deck`. The
    // deletion will be based on probability. The method should
    // have a 65% chance of deleting a random card from the
    // `Deck`. Target the deleted card by a random index
    // position."
    // ---------------------------------------------------------
    fn delete_random_card(&mut self) {
        let mut rng = thread_rng();

        // gen_bool(0.65) returns true 65% of the time -- that's
        // the whole "probability" requirement in one call.
        if self.cards.is_empty() {
            return;
        }

        if rng.gen_bool(0.65) {
            let index = rng.gen_range(0..self.cards.len());
            self.cards.remove(index);
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("Fresh deck: {} cards", deck.cards.len());
    println!("{:#?}\n", deck);

    // ---------------------------------------------------------
    // "In `main`, instantiate the `Deck`, call the method, and
    // confirm the card order is randomized."
    // ---------------------------------------------------------
    deck.shuffle();
    println!("After shuffle: {} cards (order randomized)", deck.cards.len());
    println!("{:#?}\n", deck);

    // ---------------------------------------------------------
    // "Invoke the method in `main` and confirm you see the
    // Jokers added."
    // ---------------------------------------------------------
    deck.insert_jokers();
    println!("After insert_jokers: {} cards", deck.cards.len());
    println!("{:#?}\n", deck);

    // ---------------------------------------------------------
    // "In `main`, invoke the `delete_random_card` method 10
    // times (HINT: Don't write the method out 10 times. Find
    // a more creative way to repeat the action). Print out the
    // length of `cards` vector after the operation. We should
    // expect to see the length decrease."
    // ---------------------------------------------------------
    for _ in 0..10 {
        deck.delete_random_card();
    }

    println!("After 10 delete_random_card calls: {} cards", deck.cards.len());
}