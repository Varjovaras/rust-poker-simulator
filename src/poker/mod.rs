use crate::deck::{card::Card, Deck};

use self::player::Player;
pub mod hand;
pub mod player;

pub struct Poker {
    pub deck: Deck,
    pub hand_size: u8,
    pub players: Vec<Player>,
    pub dealer: u8,
    pub small_blind: u32,
    pub big_blind: u32,
}

impl Poker {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(hand_size: u8, player_size: u8, _buy_in: u32, big_blind: u32) -> Self {
        assert!(player_size >= 2, "Poker must have at least two players");
        assert!(
            player_size <= 23,
            "Poker must have at maximum 23 players, because the deck only has 52 cards"
        );
        let deck = Deck::new();
        Self {
            deck,
            hand_size,
            players: Vec::new(),
            dealer: 0,
            small_blind: big_blind / 2,
            big_blind,
        }
    }

    #[must_use]
    pub fn new_texas_hold_em(_player_size: u8, _buy_in: u32, big_blind: u32) -> Self {
        let deck = Deck::new();
        Self {
            deck,
            hand_size: 2,
            players: Vec::new(),
            dealer: 0,
            small_blind: big_blind / 2,
            big_blind,
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle();
    }

    pub fn deal_all_players(&mut self) {
        for _ in 0..self.hand_size {
            //    let pasa = self.players.iter().
        }
    }

    pub fn deal_a_card(&mut self) -> Option<Card> {
        self.deck.cards.pop()
    }

    pub fn burn_card(&mut self) {
        self.deck.cards.pop();
    }

    // pub fn deal_hand(&mut self) {
    //     self.deck.cards.pop();
    //     self.deck.cards.pop();
    // }
}

impl Default for Poker {
    fn default() -> Self {
        Self::new_texas_hold_em(4, 0, 0)
    }
}
