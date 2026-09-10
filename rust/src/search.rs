use crate::cards::suit;
use crate::engine::{Durak, Phase};
use crate::rollout::softmax_action;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

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

fn value(game: &Durak) -> f64 {
    if game.phase != Phase::Over {
        return positional_value(game);
    }

    match game.durak {
        None => 0.5,
        Some(1) => 1.0,
        Some(_) => 0.0,
    }
}

fn simulate(game: &mut Durak, rng: &mut SmallRng) -> f64 {
    for _ in 0..TRUNCATION {
        if game.phase == Phase::Over {
            break;
        }
        let action = softmax_action(game, rng);
        game.apply(action);
    }
    value(game)
}

fn determinize(game: &Durak, player: usize, rng: &mut SmallRng) -> Durak {
    let mut state = game.clone();
    let opponent = 1 - player;
    let has_talon = !game.talon.is_empty();
    let mut unknown: Vec<_> = game.hidden_from(player).cards().collect();

    if has_talon {
        let trump_index = unknown
            .iter()
            .position(|card| *card == game.trump_card)
            .unwrap();
        unknown.remove(trump_index);
    }
    unknown.shuffle(rng);

    let opponent_size = game.hands[opponent].len();
    state.hands[opponent] = crate::hand::Hand::from_cards(unknown[..opponent_size].to_vec());

    let mut talon = unknown[opponent_size..].to_vec();
    if has_talon {
        talon.insert(0, game.trump_card);
    }
    state.talon = talon;
    state
}
