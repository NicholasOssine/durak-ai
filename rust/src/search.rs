use crate::cards::suit;
use crate::engine::Durak;
use crate::rollout::softmax_action;

const TRUNCATION: usize = 24;
const HAND_WEIGHT: f64 = 1.0;
const TRUMP_WEIGHT: f64 = 0.9;
const VALUE_SCALE: f64 = 3.0;
const EXPLORATION: f64 = 0.7;

fn trumps(game: &Durak, player: usize) -> usize {
    game.hands[player]
        .cards()
        .filter(|card| suit(*card) == game.trump)
        .count()
}
