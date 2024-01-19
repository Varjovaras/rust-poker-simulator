use super::{rank::Rank, suit::Suit};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}

impl Card {
    pub fn new(suit: Suit, value: Rank) -> Card {
        Card { suit, value }
    }
}
