use crate::cards::{Card, DECK_SIZE};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Hand {
    bits: u64,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { bits: 0 }
    }

    pub fn from_cards(cards: Vec<Card>) -> Hand {
        let mut hand = Hand::new();
        for card in cards {
            hand.add(card);
        }
        hand
    }

    pub fn add(&mut self, card: Card) {
        self.bits |= 1 << card;
    }

    pub fn remove(&mut self, card: Card) {
        self.bits &= !(1 << card);
    }

    pub fn contains(&self, card: Card) -> bool {
        self.bits & (1 << card) != 0
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn cards(&self) -> impl Iterator<Item = Card> + '_ {
        (0..DECK_SIZE as Card).filter(|card| self.contains(*card))
    }
}
