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

fn positional_value(game: &Durak) -> f64 {
    let hand_difference = game.hands[0].len() as f64 - game.hands[1].len() as f64;
    let trump_difference = trumps(game, 0) as f64 - trumps(game, 1) as f64;
    let advantage = -HAND_WEIGHT * hand_difference + TRUMP_WEIGHT * trump_difference;

    1.0 / (1.0 + (-advantage / VALUE_SCALE).exp())
}
