use super::{rank::Rank, suit::Suit};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}

impl Card {
    #[must_use]
    pub const fn new(suit: Suit, value: Rank) -> Self {
        Self { suit, value }
    }
}
