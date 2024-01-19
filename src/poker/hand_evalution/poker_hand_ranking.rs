use crate::deck::card::Card;

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
        values_as_array[card.value as usize] += 1;
    }
    values_as_array
}

fn is_straight(values: &[u8; 14]) -> bool {
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

fn is_flush(suits: &[u8; 4]) -> bool {
    suits.iter().any(|&count| count >= 5)
}

fn is_straight_flush(suits: &[u8; 4], values: &[u8; 14]) -> bool {
    is_flush(suits) && is_straight(values)
}

fn straight_is_flush(values: [u8; 5]) -> bool {
    true
}

fn is_four_of_a_kind(values: &[u8; 14]) -> bool {
    values.contains(&4)
}

//triplet bigger than pair if both are full house
fn is_full_house(values: &[u8; 14]) -> bool {
    values.contains(&3) && values.contains(&2)
}
