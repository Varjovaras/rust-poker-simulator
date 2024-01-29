use crate::deck::card::Card;

pub struct Player {
    pub hand: Vec<Card>,
    pub chips: u32,
    pub bet: u32,
    pub is_in: bool,
}

impl Player {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn new(chips: u32) -> Self {
        assert!(chips > 0, "Player must have at least one chip");
        Self {
            hand: Vec::new(),
            chips,
            bet: 0,
            is_in: true,
        }
    }

    pub fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn fold(&mut self) {
        self.is_in = false;
        self.hand = Vec::new();
    }

    pub fn call(&mut self, amount: u32) {
        self.bet += amount;
        self.chips -= amount;
    }

    pub fn raise(&mut self, amount: u32) {
        self.bet += amount;
        self.chips -= amount;
    }

    pub fn all_in(&mut self) {
        self.bet += self.chips;
        self.chips = 0;
    }

    pub fn win(&mut self, amount: u32) {
        self.chips += amount;
    }

    pub fn lose(&mut self) {
        self.chips -= self.bet;
    }
}
