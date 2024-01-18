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
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in &SUITS {
            RANKS.iter().for_each(|value| {
                cards.push(Card::new(*suit, *value));
            });
        }

        if cards.len() != 52 {
            panic!("Deck should have 52 cards");
        }
        Deck { cards }
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
