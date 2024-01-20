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
    /// Creates a new instance of `PokerHandRanking` based on the given `cards`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `cards` is less than 5.
    pub fn new(cards: &[Card]) -> Self {
        let suits_array = hand_suits_as_array(cards);
        let values_array = hand_values_as_array(cards);
        let is_flush = hand_is_flush(suits_array);
        let is_straight = hand_is_straight(values_array);

        if is_flush && is_straight && straight_is_flush(cards, values_array) {
            let index = get_index_where_straight_starts(values_array).unwrap();
            return if index == 10 {
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
fn hand_values_as_array(cards: &[Card]) -> [u8; 14] {
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

fn hand_is_straight(values: [u8; 14]) -> bool {
    let mut counter = 0;
    for value in &values[..10] {
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

#[allow(clippy::cast_possible_truncation)]
fn get_index_where_straight_starts(values: [u8; 14]) -> Option<u8> {
    assert!(hand_is_straight(values), "This should never be returned for any reason, because it should already be checked that values contains straight!");
    let mut counter = 0;
    for (i, value) in values[..10].iter().enumerate() {
        if *value != 0 {
            counter += 1;
        } else {
            counter = 0;
        }
        if counter == 5 {
            return Some(i as u8 - 4);
        }
    }
    None
}

fn hand_is_flush(suits: [u8; 4]) -> bool {
    suits.iter().any(|&count| count >= 5)
}

fn straight_is_flush(cards: &[Card], values: [u8; 14]) -> bool {
    let mut cards = cards.to_vec();
    cards.sort_by_key(|card| card.value);
    let straight_start_index = get_index_where_straight_starts(values).unwrap();
    let straight_cards: Vec<Card> = cards
        .iter()
        .filter(|card| {
            card.value as usize >= straight_start_index as usize
                && card.value as usize <= straight_start_index as usize + 4
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
