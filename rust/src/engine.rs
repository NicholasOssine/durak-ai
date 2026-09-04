use crate::cards::{Card, DECK_SIZE, beats, suit};
use crate::hand::Hand;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

pub const HAND_SIZE: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Attack(Card),
    Defend(Card),
    Take,
    End,
}

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

impl Durak {
    pub fn new(rng: &mut SmallRng) -> Durak {
        let mut deck: Vec<Card> = (0..DECK_SIZE as Card).collect();
        deck.shuffle(rng);

        let mut first_hand = deck[0..HAND_SIZE].to_vec();
        let mut second_hand = deck[HAND_SIZE..2 * HAND_SIZE].to_vec();
        first_hand.sort();
        second_hand.sort();

        let hands = [Hand::from_cards(first_hand), Hand::from_cards(second_hand)];
        let talon = deck[2 * HAND_SIZE..].to_vec();
        let trump_card = talon[0];
        let trump = suit(trump_card);
        let attacker = first_attacker(&hands, trump);
        let max_attacks = HAND_SIZE.min(hands[1 - attacker].len());

        Durak {
            hands,
            talon,
            trump_card,
            trump,
            attacker,
            table: Vec::new(),
            discard: Hand::new(),
            phase: Phase::Attack,
            max_attacks,
            durak: None,
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        if self.phase == Phase::Over {
            return Vec::new();
        }

        if self.phase == Phase::Defend {
            let defender = 1 - self.attacker;
            let attack_card = self.table[self.table.len() - 1].0;
            let mut actions = Vec::new();

            for card in self.hands[defender].cards() {
                if beats(attack_card, card, self.trump) {
                    actions.push(Action::Defend(card));
                }
            }
            actions.push(Action::Take);
            return actions;
        }

        if self.phase == Phase::Attack && self.table.is_empty() {
            let mut actions = Vec::new();
            for card in self.hands[self.attacker].cards() {
                actions.push(Action::Attack(card));
            }
            return actions;
        }

        Vec::new()
    }
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
