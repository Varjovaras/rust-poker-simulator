#[allow(clippy::wildcard_imports)]
use leptos::*;

use crate::poker::{hand::Hand, Poker};
pub mod deck;
pub mod poker;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });

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
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
            }

            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move"
        </button>
    }
}

//https://book.leptos.dev/view/02_dynamic_attributes.html
