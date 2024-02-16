use crate::deck::{card::Card, Deck};

use self::player::Player;
pub mod hand;
pub mod player;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Board {
    pub players: Vec<Player>,
    // pub dealer: u8,
    // pub big_blind: u32,
    // pub small_blind: u32,
    // pub pot: u32,
}

impl Board {
    #[must_use]
    pub fn new(players: u8) -> Self {
        Self {
            players: (0..players).map(Player::new).collect(),
        }
    }
}

// impl Default for Board {
//     fn default() -> Self {
//         Self {
//             players: Vec::new(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Poker {
    pub deck: Deck,
    pub hand_size: u8,
    pub board: Board,
}

impl Poker {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(hand_size: u8, player_size: u8, _buy_in: u32) -> Self {
        assert!(player_size >= 2, "Poker must have at least two players");
        assert!(
            player_size <= 23,
            "Poker must have at maximum 23 players, because the deck only has 52 cards"
        );
        let deck = Deck::new();
        Self {
            deck,
            hand_size,
            board: Board::new(player_size),
        }
    }

    #[must_use]
    pub fn new_texas_hold_em(player_size: u8, _buy_in: u32) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        Self {
            deck,
            hand_size: 2,
            board: Board::new(player_size),
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle();
    }

    pub fn new_deck(&mut self) {
        self.deck = Deck::new();
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn deal_all_players(&mut self) {
        // Clear all players' hands
        for player in &mut self.board.players {
            player.hand.clear();
        }

        // Deal new cards
        for _ in 0..self.hand_size {
            for player in &mut self.board.players {
                player
                    .hand
                    .push(self.deck.cards.pop().expect("No more cards in the deck"));
            }
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
        Self::new_texas_hold_em(4, 0)
    }
}
