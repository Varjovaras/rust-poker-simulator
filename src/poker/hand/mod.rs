use crate::deck::card::Card;

use self::poker_hand_ranking::PokerHandRanking;
pub mod poker_hand_ranking;

#[derive(Debug)]
pub struct Hand {
    pub high_card: Card,
    pub hand_value: PokerHandRanking,
}

impl Hand {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]

    pub fn new(cards_in_hand: &[Card], cards_on_table: &[Card]) -> Self {
        assert!(
            !cards_in_hand.is_empty(),
            "Hand must have at least one card"
        );
        let high_card = get_highest_card(cards_in_hand).expect("Hand must have at least one card");
        let mut hand: Vec<Card> = Vec::from(cards_in_hand);
        hand.extend_from_slice(cards_on_table);

        let hand_value = PokerHandRanking::new(&hand);

        Self {
            high_card,
            hand_value,
        }
    }
}

fn get_highest_card(cards: &[Card]) -> Option<Card> {
    cards.iter().max_by_key(|card| card.value).copied()
}
