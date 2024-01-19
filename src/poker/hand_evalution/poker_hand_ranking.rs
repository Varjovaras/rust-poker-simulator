use crate::deck::{card::Card, rank::Rank};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
    pub fn new(cards: &[Card]) -> PokerHandRanking {
        let suits_array = hand_suits_as_array(cards);
        let values_array = hand_values_as_array(cards);
        let is_flush = hand_is_flush(&suits_array);
        let is_straight = hand_is_straight(&values_array);

        if is_flush && is_straight && straight_is_flush(cards, &values_array) {
            let index = get_index_where_straight_starts(&values_array).unwrap();
            if index == 10 {
                return PokerHandRanking::RoyalFlush;
            } else {
                return PokerHandRanking::StraightFlush;
            }
        }

        PokerHandRanking::HighCard
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

fn hand_is_straight(values: &[u8; 14]) -> bool {
    let mut counter = 0;
    for value in values.iter() {
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

fn get_index_where_straight_starts(values: &[u8; 14]) -> Option<u8> {
    if !hand_is_straight(values) {
        panic!("This should never be called for any reason, because it should already be checked it contains straight!")
    }
    let mut counter = 0;
    for (i, value) in values.iter().enumerate() {
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

fn hand_is_flush(suits: &[u8; 4]) -> bool {
    suits.iter().any(|&count| count >= 5)
}

fn straight_is_flush(cards: &[Card], values: &[u8; 14]) -> bool {
    let mut cards = cards.to_vec();
    cards.sort_by_key(|card| card.value);
    let straight_start_index = get_index_where_straight_starts(values).unwrap();
    let straight_cards: Vec<Card> = cards
        .iter()
        .filter(|card| {
            card.value as usize >= straight_start_index as usize
                && card.value as usize <= straight_start_index as usize + 4
        })
        .cloned()
        .collect();

    hand_is_flush(&hand_suits_as_array(&straight_cards))
}

fn is_four_of_a_kind(values: &[u8; 14]) -> bool {
    values.contains(&4)
}

//triplet bigger than pair if both are full house
fn is_full_house(values: &[u8; 14]) -> bool {
    values.contains(&3) && values.contains(&2)
}
