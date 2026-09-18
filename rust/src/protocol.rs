use crate::cards::{Card, DECK_SIZE, suit};
use crate::engine::{Action, Durak, Phase};
use crate::hand::Hand;

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

fn parse_hand(text: &str) -> Option<Hand> {
    let mask: u64 = text.parse().ok()?;
    let mut hand = Hand::new();

    for card in 0..DECK_SIZE as Card {
        if mask & (1u64 << card) != 0 {
            hand.add(card);
        }
    }
    Some(hand)
}

fn parse_state(fields: &[&str]) -> Option<Durak> {
    if fields.len() != 9 {
        return None;
    }

    let talon_len = fields[3].parse().ok()?;
    let trump_card = fields[4].parse().ok()?;

    Some(Durak {
        hands: [parse_hand(fields[0])?, parse_hand(fields[1])?],
        talon: vec![0; talon_len],
        trump_card,
        trump: suit(trump_card),
        attacker: fields[6].parse().ok()?,
        table: parse_table(fields[5])?,
        discard: parse_hand(fields[2])?,
        phase: parse_phase(fields[7])?,
        max_attacks: fields[8].parse().ok()?,
        durak: None,
    })
}
