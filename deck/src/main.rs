#[derive(Debug)]
struct Deck {
    decks: Vec<String>,
}

fn main() {
    let deck = Deck { decks: vec![] };
    println!("decks: {:?}", deck.decks);
}
