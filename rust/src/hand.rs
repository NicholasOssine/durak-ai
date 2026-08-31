use crate::cards::Card;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn from_cards(cards: Vec<Card>) -> Hand {
        Hand { cards }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove(&mut self, card: Card) {
        let index = self.cards.iter().position(|held| *held == card).unwrap();
        self.cards.remove(index);
    }
}
