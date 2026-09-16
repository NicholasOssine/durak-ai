use crate::engine::{Durak, Phase};

fn utility(game: &Durak) -> i8 {
    match game.durak {
        None => 0,
        Some(1) => 1,
        Some(_) => -1,
    }
}

fn minimax(game: &Durak) -> i8 {
    if game.phase == Phase::Over {
        return utility(game);
    }

    if game.current_player() == 0 {
        let mut value = i8::MIN;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            value = value.max(minimax(&child));
        }
        value
    } else {
        let mut value = i8::MAX;
        for action in game.get_actions() {
            let mut child = game.clone();
            child.apply(action);
            value = value.min(minimax(&child));
        }
        value
    }
}
