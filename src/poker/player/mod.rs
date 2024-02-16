use crate::deck::card::Card;

#[derive(Debug, Clone, Default)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub id: u8,
    // pub chips: u32,
    // pub bet: u32,
    // pub is_in: bool,
}

impl Player {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(
        // name: String,
        id: u8,
    ) -> Self {
        // assert!(chips > 0, "Player must have at least one chip");
        Self {
            name: id.to_string(),
            hand: Vec::new(),
            id, // chips,
                // bet: 0,
                // is_in: true,
        }
    }

    pub fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    #[must_use]
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

    #[must_use]
    pub fn get_hand_as_str(&self) -> String {
        let mut hand_str = String::new();
        for card in &self.hand {
            hand_str.push_str(&card.to_custom_string());
            hand_str.push_str(" | "); // Add a separator between cards
        }
        hand_str.trim_end_matches(" | ").to_string() // Remove trailing separator
    }

    // pub fn fold(&mut self) {
    //     self.is_in = false;
    //     self.hand = Vec::new();
    // }

    // pub fn call(&mut self, amount: u32) {
    //     self.bet += amount;
    //     self.chips -= amount;
    // }

    // pub fn raise(&mut self, amount: u32) {
    //     self.bet += amount;
    //     self.chips -= amount;
    // }

    // pub fn all_in(&mut self) {
    //     self.bet += self.chips;
    //     self.chips = 0;
    // }

    // pub fn win(&mut self, amount: u32) {
    //     self.chips += amount;
    // }

    // pub fn lose(&mut self) {
    //     self.chips -= self.bet;
    // }
}

// impl Default for Player {
//     fn default() -> Self {
//         Self {
//             hand: Vec::new(),
//             id: 0,
//             //     chips: 100,
//             //     bet: 0,
//             //     is_in: true,
//             // }
//         }
//     }
// }
