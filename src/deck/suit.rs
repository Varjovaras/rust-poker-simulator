use std::fmt;
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    #[default]
    Spade,
}

impl Suit {
    #[must_use]
    pub fn _from_str(suit: &str) -> Option<Self> {
        match suit {
            "Heart" => Some(Self::Heart),
            "Diamond" => Some(Self::Diamond),
            "Club" => Some(Self::Club),
            "Spade" => Some(Self::Spade),
            _ => None,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Heart => write!(f, "Heart"),
            Self::Diamond => write!(f, "Diamond"),
            Self::Club => write!(f, "Club"),
            Self::Spade => write!(f, "Spade"),
        }
    }
}

pub const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
