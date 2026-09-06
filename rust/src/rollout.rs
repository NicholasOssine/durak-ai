use crate::cards::{Card, rank, suit};
use crate::engine::{Action, Durak};
use rand::RngExt;
use rand::rngs::SmallRng;

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

fn action_weights(actions: &[Action], trump: u8) -> Vec<f64> {
    let mut weights = Vec::new();
    for &action in actions {
        weights.push(action_weight(action, trump));
    }
    weights
}

pub fn softmax_action(game: &Durak, rng: &mut SmallRng) -> Action {
    let actions = game.get_actions();
    if actions.len() == 1 {
        return actions[0];
    }

    let weights = action_weights(&actions, game.trump);
    let total: f64 = weights.iter().sum();
    let target = rng.random::<f64>() * total;
    let mut running = 0.0;

    for index in 0..actions.len() {
        running += weights[index];
        if running >= target {
            return actions[index];
        }
    }

    actions[actions.len() - 1]
}
