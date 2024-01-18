#[derive(Copy, Clone, Debug, Default)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    #[default]
    Spade,
}

pub const SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
