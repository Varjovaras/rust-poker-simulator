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
    pub fn _from_str(rank: &str) -> Option<Rank> {
        match rank {
            "Two" => Some(Rank::Two),
            "Three" => Some(Rank::Three),
            "Four" => Some(Rank::Four),
            "Five" => Some(Rank::Five),
            "Six" => Some(Rank::Six),
            "Seven" => Some(Rank::Seven),
            "Eight" => Some(Rank::Eight),
            "Nine" => Some(Rank::Nine),
            "Ten" => Some(Rank::Ten),
            "Jack" => Some(Rank::Jack),
            "Queen" => Some(Rank::Queen),
            "King" => Some(Rank::King),
            "Ace" => Some(Rank::Ace),
            _ => None,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
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
