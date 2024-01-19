use crate::deck::card::Card;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum _HandValues {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub struct HandValue {
    pub high_card: Card,
    pub hand_value: _HandValues,
}

impl HandValue {
    fn _new(cards_in_hand: &[Card], _cards_on_table: &[Card]) -> HandValue {
        let hand_value = _HandValues::HighCard;
        let high_card = _get_highest_card(cards_in_hand).unwrap();

        HandValue {
            high_card,
            hand_value,
        }
    }
}

fn _get_highest_card(cards: &[Card]) -> Option<Card> {
    cards.iter().max_by_key(|card| card.value).cloned()
}
