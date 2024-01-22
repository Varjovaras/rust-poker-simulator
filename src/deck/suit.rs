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
    /// Parses a string representation of a suit.
    ///
    /// # Arguments
    ///
    /// * `suit` - A string slice representing the suit.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the input string does not match any valid suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use deck::Suit;
    ///
    /// let suit = Suit::parse_str("Heart").unwrap();
    /// assert_eq!(suit, Suit::Heart);
    /// ```
    pub fn parse_str(suit: &str) -> Result<Self, String> {
        match suit {
            "Heart" | "H" | "h" => Ok(Self::Heart),
            "Diamond" | "D" | "d" => Ok(Self::Diamond),
            "Club" | "C" | "c" => Ok(Self::Club),
            "Spade" | "S" | "s" => Ok(Self::Spade),
            _ => Err(format!("Invalid suit: {suit}")),
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
