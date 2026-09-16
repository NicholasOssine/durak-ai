use crate::engine::{Action, Durak, Phase};

pub const NODES: u32 = 120_000;

fn utility(game: &Durak) -> i8 {
    match game.durak {
        None => 0,
        Some(1) => 1,
        Some(_) => -1,
    }
}

fn minimax(game: &Durak, mut alpha: i8, mut beta: i8, budget: &mut u32) -> Option<i8> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;

    if game.phase == Phase::Over {
        return Some(utility(game));
    }

    if game.current_player() == 0 {
        let mut value = i8::MIN;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            value = value.max(minimax(&child, alpha, beta, budget)?);
            alpha = alpha.max(value);
            if value >= beta {
                break;
            }
        }
        Some(value)
    } else {
        let mut value = i8::MAX;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            value = value.min(minimax(&child, alpha, beta, budget)?);
            beta = beta.min(value);
            if value <= alpha {
                break;
            }
        }
        Some(value)
    }
}

pub fn best_action(game: &Durak, nodes: u32) -> Option<Action> {
    let mut budget = nodes;
    let mut chosen_action = None;

    if game.current_player() == 0 {
        let mut chosen_value = i8::MIN;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            let value = minimax(&child, chosen_value, i8::MAX, &mut budget)?;
            if value > chosen_value {
                chosen_action = Some(action);
                chosen_value = value;
            }
        }
    } else {
        let mut chosen_value = i8::MAX;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            let value = minimax(&child, i8::MIN, chosen_value, &mut budget)?;
            if value < chosen_value {
                chosen_action = Some(action);
                chosen_value = value;
            }
        }
    }

    chosen_action
}
