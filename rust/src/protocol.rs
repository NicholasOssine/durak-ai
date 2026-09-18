use crate::cards::Card;
use crate::engine::{Action, Phase};

fn format_action(action: Action) -> String {
    match action {
        Action::Attack(card) => format!("a{card}"),
        Action::Defend(card) => format!("d{card}"),
        Action::Take => "t".to_string(),
        Action::End => "e".to_string(),
    }
}

fn parse_phase(text: &str) -> Option<Phase> {
    match text {
        "ATTACK" => Some(Phase::Attack),
        "DEFEND" => Some(Phase::Defend),
        "TAKING" => Some(Phase::Taking),
        "OVER" => Some(Phase::Over),
        _ => None,
    }
}
