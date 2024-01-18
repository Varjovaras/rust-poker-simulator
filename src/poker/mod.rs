use self::deck::Deck;

pub mod deck;

pub struct Poker {
    pub deck: Deck,
}

impl Poker {
    pub fn new() -> Poker {
        let deck = Deck::new();
        Poker { deck }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle();
    }
}
