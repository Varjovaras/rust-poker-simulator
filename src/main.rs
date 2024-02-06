pub mod deck;
pub mod frontend;
pub mod poker;

fn main() {
    frontend::mount();
}

//https://book.leptos.dev/view/02_dynamic_attributes.html

/*

*
    // let mut poker = Poker::new_texas_hold_em(4, 0); // Call
    // for i in 0..1_000 {
    //     poker.deck.shuffle();
    //     let hand_value = Hand::new(&poker.deck.cards[0..2], &poker.deck.cards[2..7]);

    //     if i % 10000 == 0 {
    //         println!("{:?}", poker.deck.cards[0..7].to_vec());
    //         println!("{hand_value:?}");
    //     }
    // }

    // println!("{:?}", poker.deck.cards[0..7].to_vec());
*/
