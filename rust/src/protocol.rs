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

fn parse_table(text: &str) -> Option<Vec<(Card, Option<Card>)>> {
    if text == "-" {
        return Some(Vec::new());
    }

    let mut table = Vec::new();
    for entry in text.split(',') {
        let (attack, defence) = entry.split_once(':')?;
        let attack = attack.parse().ok()?;
        let defence = if defence == "-" {
            None
        } else {
            Some(defence.parse().ok()?)
        };
        table.push((attack, defence));
    }
    Some(table)
}
