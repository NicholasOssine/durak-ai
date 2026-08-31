pub const HAND_SIZE: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Attack,
    Defend,
    Taking,
    Over,
}
