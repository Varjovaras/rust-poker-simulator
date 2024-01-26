use std::fmt;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
/*
* Ace is both 0 and 13 because of the way the `Rank` enum is used in the `Hand` struct.
* So enum starts at Two = 1
*/
pub enum Rank {
    #[default]
    Two = 1,
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
    /// Parses a string representation of a rank and returns the corresponding `Rank` enum variant.
    ///
    /// # Arguments
    ///
    /// * `rank` - A string representing the rank.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the input string does not match any valid rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use deck::Rank;
    ///
    /// let rank = Rank::parse_str("Two");
    /// assert_eq!(rank, Ok(Rank::Two));
    /// ```
    pub fn parse_str(rank: &str) -> Result<Self, String> {
        match rank {
            "Two" | "2" => Ok(Self::Two),
            "Three" | "3" => Ok(Self::Three),
            "Four" | "4" => Ok(Self::Four),
            "Five" | "5" => Ok(Self::Five),
            "Six" | "6" => Ok(Self::Six),
            "Seven" | "7" => Ok(Self::Seven),
            "Eight" | "8" => Ok(Self::Eight),
            "Nine" | "9" => Ok(Self::Nine),
            "Ten" | "10" => Ok(Self::Ten),
            "Jack" | "J" => Ok(Self::Jack),
            "Queen" | "Q" => Ok(Self::Queen),
            "King" | "K" => Ok(Self::King),
            "Ace" | "A" => Ok(Self::Ace),
            _ => Err(format!("Invalid rank: {rank}")),
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
