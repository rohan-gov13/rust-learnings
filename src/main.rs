use rand::{thread_rng, seq::SliceRandom};


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

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    fn deal(&mut self, num_card: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_card)
    }
}
 
fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let cards = deck.deal(3);
    
    println!("deck {:#?}", deck);
    println!("removed cards from deck {:#?}", cards);
}
