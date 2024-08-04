#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "spades", "diamonds"];
        let values = ["ace", "two", "three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card)
            }
        }
        
        Deck { cards }

    }
}
 
fn main() {
    let deck = Deck::new();
    
    println!("deck {:#?}", deck);
}
