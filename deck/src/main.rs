use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suites = ["Hearts", "Spades", "Clubs", "Diamonds"];
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];
        let mut cards = vec![];
        for suit in suites {
            for val in values {
                let card = format!("{} of {}", suit, val);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("decks: {:#?}", deck);
}
