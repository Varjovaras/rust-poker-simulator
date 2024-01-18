use crate::deck::Deck;
pub mod hand_value;

pub struct Poker {
    pub deck: Deck,
    pub hand_size: u8,
}

impl Poker {
    pub fn new(hand_size: u8) -> Poker {
        let deck = Deck::new();
        Poker { deck, hand_size }
    }

    pub fn new_texas_hold_em() -> Poker {
        let deck = Deck::new();
        Poker { deck, hand_size: 2 }
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
