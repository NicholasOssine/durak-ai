use crate::cards::{Card, suit};
use crate::hand::Hand;

pub const HAND_SIZE: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Attack,
    Defend,
    Taking,
    Over,
}

#[derive(Clone)]
pub struct Durak {
    pub hands: [Hand; 2],
    pub talon: Vec<Card>,
    pub trump_card: Card,
    pub trump: u8,
    pub attacker: usize,
    pub table: Vec<(Card, Option<Card>)>,
    pub discard: Hand,
    pub phase: Phase,
    pub max_attacks: usize,
    pub durak: Option<usize>,
}

fn first_attacker(hands: &[Hand; 2], trump: u8) -> usize {
    let mut best_player = 0;
    let mut best_trump = None;

    for player in 0..2 {
        for card in hands[player].cards() {
            if suit(card) == trump && (best_trump.is_none() || card < best_trump.unwrap()) {
                best_trump = Some(card);
                best_player = player;
            }
        }
    }

    best_player
}
