use std::fmt;

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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card {{ suit: {}, value: {} }}", self.suit, self.value)
    }
}
