use std::fmt::format;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut cards = vec![];

    for suit in &suits {
        for value in &values {
            let card = format!("{value} of {suit}");
            cards.push(card);
        }
    }

    let deck: Deck = Deck { cards: vec![] };

    println!("Your deck is: {:?}", deck);
}
