use super::{rank::Rank, suit::Suit};

#[derive(Copy, Clone, Debug)]
pub struct Card {
    _suit: Suit,
    _value: Rank,
}

impl Card {
    pub fn new(suit: Suit, value: Rank) -> Card {
        Card {
            _suit: suit,
            _value: value,
        }
    }
}
