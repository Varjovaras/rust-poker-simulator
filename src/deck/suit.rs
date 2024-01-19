use std::fmt;
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    #[default]
    Spade,
}

impl Suit {
    pub fn _from_str(suit: &str) -> Option<Suit> {
        match suit {
            "Heart" => Some(Suit::Heart),
            "Diamond" => Some(Suit::Diamond),
            "Club" => Some(Suit::Club),
            "Spade" => Some(Suit::Spade),
            _ => None,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
            Suit::Spade => write!(f, "Spade"),
        }
    }
}

pub const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
