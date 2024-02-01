pub mod card;
pub mod rank;
pub mod suit;
use self::card::Card;
use self::suit::SUITS;

use self::rank::RANKS;
use rand::seq::SliceRandom;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in &SUITS {
            for value in &RANKS {
                cards.push(Card::new(*suit, *value));
            }
        }
        assert!(cards.len() == 52, "Deck should have 52 cards");
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
