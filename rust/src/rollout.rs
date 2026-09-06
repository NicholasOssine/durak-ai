use crate::cards::{Card, rank, suit};

const TEMPERATURE: f64 = 2.0;

fn cost(card: Card, trump: u8) -> u8 {
    rank(card) + if suit(card) == trump { 9 } else { 0 }
}

fn softmax_weight(action_cost: u8) -> f64 {
    (-(action_cost as f64) / TEMPERATURE).exp()
}
