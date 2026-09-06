use crate::cards::{Card, rank, suit};

fn cost(card: Card, trump: u8) -> u8 {
    rank(card) + if suit(card) == trump { 9 } else { 0 }
}
