use crate::poker::Poker;

pub mod deck;
pub mod poker;

fn main() {
    let mut poker = Poker::new_texas_hold_em(); // Call
    for i in 0..1_000_000 {
        poker.deck.shuffle();
        if i % 10000 == 0 {
            println!("{:?}", poker.deck.cards[0..5].to_vec());
        }
    }

    println!("{:?}", poker.deck.cards[0..5].to_vec());

    println!("Hello, world!");
}
