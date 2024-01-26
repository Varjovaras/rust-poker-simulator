use crate::deck::{card::Card, rank::Rank};
use bytecount;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub enum PokerHandRanking {
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

impl PokerHandRanking {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(cards: &[Card]) -> Self {
        assert!(
            cards.len() >= 5,
            "There must be at least 5 cards in the hand!"
        );
        assert!(
            no_duplicate_cards(cards),
            "There must be no duplicate cards in the hand!"
        );
        Self::hand_value_calculator(cards)
    }

    fn hand_value_calculator(cards: &[Card]) -> Self {
        let suits_array = hand_suits_as_array(cards);
        let values_array = hand_values_as_array(cards);
        let is_flush = hand_is_flush(suits_array);
        let is_straight = hand_is_straight(values_array);

        if is_flush && is_straight && straight_is_flush(cards) {
            let index = get_index_where_straight_starts(values_array).expect(
                "This should never be returned for any reason, because it should already be checked that values contains straight!",
            );
            return if index == 9 {
                Self::RoyalFlush
            } else {
                Self::StraightFlush
            };
        }
        if is_four_of_a_kind(values_array) {
            return Self::FourOfAKind;
        }
        if is_full_house(values_array) {
            return Self::FullHouse;
        }
        if is_flush {
            return Self::Flush;
        }
        if is_straight {
            return Self::Straight;
        }

        if is_three_of_kind(values_array) {
            return Self::ThreeOfAKind;
        }

        if is_two_pair(values_array) {
            return Self::TwoPair;
        }

        if is_pair(values_array) {
            return Self::Pair;
        }

        Self::HighCard
    }
}

//Heart, Diamond, Club, Spade
//Heart is the u8 in [0]
//Diamond is the u8 in [1]
//Club is the u8 in [2]
//Spade is the u8 in [3]
#[must_use]
fn hand_suits_as_array(cards: &[Card]) -> [u8; 4] {
    let mut suits_as_array = [0; 4];
    for card in cards {
        suits_as_array[card.suit as usize] += 1;
    }
    suits_as_array
}

//first element is Ace as 1
//then from 2 to K
//last element is ace as 14
#[must_use]
pub fn hand_values_as_array(cards: &[Card]) -> [u8; 14] {
    let mut values_as_array = [0; 14];
    for card in cards {
        if card.value == Rank::Ace {
            values_as_array[0] += 1;
            values_as_array[13] += 1;
        } else {
            values_as_array[card.value as usize] += 1;
        }
    }
    values_as_array
}

fn no_duplicate_cards(cards: &[Card]) -> bool {
    for i in 0..cards.len() {
        for j in i + 1..cards.len() {
            if cards[i] == cards[j] {
                return false;
            }
        }
    }

    true
}
fn hand_is_straight(values: [u8; 14]) -> bool {
    let mut counter = 0;
    for value in &values {
        if *value != 0 {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == 5 {
            return true;
        }
    }
    false
}

fn get_index_where_straight_starts(values: [u8; 14]) -> Option<usize> {
    assert!(hand_is_straight(values), "This should never be returned for any reason, because it should already be checked that values contains straight!");
    let mut counter = 0;
    for (i, value) in values.iter().enumerate() {
        if *value != 0 {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == 5 {
            return Some(i - 4);
        }
    }
    None
}

fn hand_is_flush(suits: [u8; 4]) -> bool {
    suits.iter().any(|&count| count >= 5)
}

fn straight_is_flush(cards: &[Card]) -> bool {
    let mut cards = cards.to_vec();
    cards.sort_by_key(|card| card.value);
    let values = hand_values_as_array(&cards);
    let straight_start_index = get_index_where_straight_starts(values).expect(
        "This should never be returned for any reason, because it should already be checked that the cards contain flush and a straight.",
    );
    let straight_cards: Vec<Card> = cards
        .iter()
        .filter(|card| {
            card.value as usize >= straight_start_index
                && card.value as usize <= straight_start_index + 4
        })
        .copied()
        .collect();

    hand_is_flush(hand_suits_as_array(&straight_cards))
}

fn is_four_of_a_kind(values: [u8; 14]) -> bool {
    values.contains(&4)
}

//triplet bigger than pair if both are full house
fn is_full_house(values: [u8; 14]) -> bool {
    values.contains(&3) && values.contains(&2)
}

fn is_three_of_kind(values: [u8; 14]) -> bool {
    values.contains(&3)
}

fn is_two_pair(values: [u8; 14]) -> bool {
    bytecount::count(&values, 2) == 2
}

fn is_pair(values: [u8; 14]) -> bool {
    values.contains(&2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::suit::Suit;

    #[test]
    fn test_hand_is_straight() {
        let values = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0];
        assert!(hand_is_straight(values));
    }

    #[test]
    fn test_get_index_where_straight_starts() {
        let values = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0];
        assert_eq!(get_index_where_straight_starts(values), Some(5));
    }

    #[test]
    fn test_hand_is_flush() {
        let suits = [0, 5, 0, 0];
        assert!(hand_is_flush(suits));
        let suits = [3, 4, 2, 1];
        assert!(!hand_is_flush(suits));
    }

    #[test]
    fn test_straight_is_flush() {
        let cards = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Queen,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Jack,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Ten,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Nine,
            },
        ];
        assert!(straight_is_flush(&cards));
    }

    #[test]
    fn test_is_four_of_a_kind() {
        let values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4];
        assert!(is_four_of_a_kind(values));
    }

    #[test]
    fn test_is_full_house() {
        let values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0];
        assert!(is_full_house(values));
    }

    #[test]
    fn test_is_three_of_kind() {
        let values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0];
        assert!(is_three_of_kind(values));
    }

    #[test]
    fn test_is_two_pair() {
        let values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0];
        assert!(is_two_pair(values));
    }

    #[test]
    fn test_is_pair() {
        let values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0];
        assert!(is_pair(values));
    }

    #[test]
    fn test_royal_flush() {
        let royal_flush = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::Ace,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Queen,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Jack,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Ten,
            },
        ];

        assert_eq!(
            PokerHandRanking::new(&royal_flush),
            PokerHandRanking::RoyalFlush
        );
    }

    #[test]
    fn test_straight_flush() {
        //normal straight flush from 5 to 9
        let straight_flush = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Eight,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Seven,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Six,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Five,
            },
        ];
        println!(
            "{:?}",
            PokerHandRanking::hand_value_calculator(&straight_flush)
        );
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&straight_flush),
            PokerHandRanking::StraightFlush
        );
        //normal straight flush from 9 to king
        let straight_flush = vec![
            Card {
                suit: Suit::Club,
                value: Rank::King,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Queen,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Jack,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Nine,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&straight_flush),
            PokerHandRanking::StraightFlush
        );

        //normal straight flush from 9 to king
        let straight_flush = vec![
            Card {
                suit: Suit::Club,
                value: Rank::King,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Queen,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Jack,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Nine,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&straight_flush),
            PokerHandRanking::StraightFlush
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        // four of a kind with 9s
        let four_of_a_kind = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Spade,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Five,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&four_of_a_kind),
            PokerHandRanking::FourOfAKind
        );

        // four of a kind with Kings
        let four_of_a_kind = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
            Card {
                suit: Suit::Club,
                value: Rank::King,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::King,
            },
            Card {
                suit: Suit::Spade,
                value: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Five,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&four_of_a_kind),
            PokerHandRanking::FourOfAKind
        );
    }

    #[test]
    fn test_full_house() {
        // full house with 3 Kings and 2 Queens
        let full_house = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
            Card {
                suit: Suit::Club,
                value: Rank::King,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::King,
            },
            Card {
                suit: Suit::Spade,
                value: Rank::Queen,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Queen,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&full_house),
            PokerHandRanking::FullHouse
        );

        // full house with 3 Aces and 2 Kings
        let full_house = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::Ace,
            },
            Card {
                suit: Suit::Club,
                value: Rank::Ace,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Ace,
            },
            Card {
                suit: Suit::Spade,
                value: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&full_house),
            PokerHandRanking::FullHouse
        );
    }

    #[test]
    fn test_flush() {
        // flush with all Hearts
        let flush = vec![
            Card {
                suit: Suit::Heart,
                value: Rank::Ace,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::King,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Ten,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Seven,
            },
            Card {
                suit: Suit::Heart,
                value: Rank::Two,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&flush),
            PokerHandRanking::Flush
        );

        // flush with all Diamonds
        let flush = vec![
            Card {
                suit: Suit::Diamond,
                value: Rank::Nine,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Eight,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Six,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Four,
            },
            Card {
                suit: Suit::Diamond,
                value: Rank::Two,
            },
        ];
        assert_eq!(
            PokerHandRanking::hand_value_calculator(&flush),
            PokerHandRanking::Flush
        );
    }
}
