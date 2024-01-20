use std::fmt;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub enum Rank {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    #[must_use]
    pub fn _from_str(rank: &str) -> Option<Self> {
        match rank {
            "Two" => Some(Self::Two),
            "Three" => Some(Self::Three),
            "Four" => Some(Self::Four),
            "Five" => Some(Self::Five),
            "Six" => Some(Self::Six),
            "Seven" => Some(Self::Seven),
            "Eight" => Some(Self::Eight),
            "Nine" => Some(Self::Nine),
            "Ten" => Some(Self::Ten),
            "Jack" => Some(Self::Jack),
            "Queen" => Some(Self::Queen),
            "King" => Some(Self::King),
            "Ace" => Some(Self::Ace),
            _ => None,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Two => write!(f, "Two"),
            Self::Three => write!(f, "Three"),
            Self::Four => write!(f, "Four"),
            Self::Five => write!(f, "Five"),
            Self::Six => write!(f, "Six"),
            Self::Seven => write!(f, "Seven"),
            Self::Eight => write!(f, "Eight"),
            Self::Nine => write!(f, "Nine"),
            Self::Ten => write!(f, "Ten"),
            Self::Jack => write!(f, "Jack"),
            Self::Queen => write!(f, "Queen"),
            Self::King => write!(f, "King"),
            Self::Ace => write!(f, "Ace"),
        }
    }
}

pub const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];
