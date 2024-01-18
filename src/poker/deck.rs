use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, Default)]
enum Suit {
    Heart,
    Diamond,
    Club,
    #[default]
    Spade,
}

#[derive(Copy, Clone, Debug, Default)]
enum Rank {
    #[default]
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    suit: Suit,
    value: Rank,
}

pub struct Deck {
    cards: Vec<Card>,
}

const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in SUITS {
            RANKS.iter().for_each(|value| {
                cards.push(Card {
                    suit,
                    value: *value,
                });
            });
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
