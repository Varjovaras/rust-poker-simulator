use crate::deck::Deck;
pub mod hand_evalution;

pub struct Poker {
    pub deck: Deck,
    pub hand_size: u8,
}

impl Poker {
    #[must_use]
    pub fn new(hand_size: u8) -> Self {
        let deck = Deck::new();
        Self { deck, hand_size }
    }

    #[must_use]
    pub fn new_texas_hold_em() -> Self {
        let deck = Deck::new();
        Self { deck, hand_size: 2 }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle();
    }

    pub fn deal(&mut self) {
        self.deck.cards.pop();
    }

    pub fn deal_hand(&mut self) {
        self.deck.cards.pop();
        self.deck.cards.pop();
    }
}

impl Default for Poker {
    fn default() -> Self {
        Self::new_texas_hold_em()
    }
}
