use crate::engine::Durak;

fn utility(game: &Durak) -> i8 {
    match game.durak {
        None => 0,
        Some(1) => 1,
        Some(_) => -1,
    }
}
