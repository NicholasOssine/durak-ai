use crate::cards::Card;
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
