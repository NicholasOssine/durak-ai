use crate::cards::{Card, rank, suit};
use crate::engine::Action;

const TEMPERATURE: f64 = 2.0;
const TAKE_COST: u8 = 99;

fn cost(card: Card, trump: u8) -> u8 {
    rank(card) + if suit(card) == trump { 9 } else { 0 }
}

fn softmax_weight(action_cost: u8) -> f64 {
    (-(action_cost as f64) / TEMPERATURE).exp()
}

fn action_weight(action: Action, trump: u8) -> f64 {
    let action_cost = match action {
        Action::Attack(card) | Action::Defend(card) => cost(card, trump),
        Action::Take | Action::End => TAKE_COST,
    };
    softmax_weight(action_cost)
}
