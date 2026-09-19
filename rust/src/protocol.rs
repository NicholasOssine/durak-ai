use crate::cards::{Card, DECK_SIZE, suit};
use crate::engine::{Action, Durak, MAX_TALON, Phase};
use crate::hand::Hand;
use crate::search;
use rand::rngs::SmallRng;
use std::time::Duration;

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
    if talon_len > MAX_TALON {
        return None;
    }
    let trump_card = fields[4].parse().ok()?;

    Some(Durak {
        hands: [parse_hand(fields[0])?, parse_hand(fields[1])?],
        talon: [0; MAX_TALON],
        talon_len,
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

pub fn respond(line: &str, rng: &mut SmallRng) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.first() != Some(&"move") || fields.len() != 11 {
        return "error expected: move <hand0> <hand1> <discard> <talon_len> <trump_card> <table> <attacker> <phase> <max_attacks> <ms>".to_string();
    }

    let game = match parse_state(&fields[1..10]) {
        Some(game) => game,
        None => return "error malformed state".to_string(),
    };
    let millis = match fields[10].parse::<u64>() {
        Ok(millis) => millis,
        Err(_) => return "error malformed budget".to_string(),
    };
    if game.get_actions().is_empty() {
        return "error no legal actions".to_string();
    }

    let action = search::ismcts_action(&game, Duration::from_millis(millis), rng);
    format_action(action)
}
