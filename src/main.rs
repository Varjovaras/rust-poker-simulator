use leptos::{mount_to_body, view};

use crate::poker::{hand::Hand, Poker};
pub mod deck;
pub mod poker;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <p>"Hello, world!"</p>
            <div>ali ali :D</div>
        }
    });

    // let mut poker = Poker::new_texas_hold_em(4, 0); // Call
    // for i in 0..1_000_000 {
    //     poker.deck.shuffle();
    //     let hand_value = Hand::new(&poker.deck.cards[0..2], &poker.deck.cards[2..7]);

    //     if i % 10000 == 0 {
    //         println!("{:?}", poker.deck.cards[0..7].to_vec());
    //         println!("{hand_value:?}");
    //     }
    // }

    // println!("{:?}", poker.deck.cards[0..7].to_vec());
}
