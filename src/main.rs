use crate::poker::Poker;

pub mod poker;

fn main() {
    let mut poker = Poker::new(); // Call
    poker.deck.shuffle();

    println!("Hello, world!");
}
