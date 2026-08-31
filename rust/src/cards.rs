pub type Card = u8;

pub const DECK_SIZE: usize = 36;

pub fn rank(card: Card) -> u8 {
    card / 4
}

pub fn suit(card: Card) -> u8 {
    card % 4
}

pub fn beats(attack: Card, defence: Card, trump: u8) -> bool {
    if suit(defence) == suit(attack) {
        return rank(defence) > rank(attack);
    }
    suit(defence) == trump
}
