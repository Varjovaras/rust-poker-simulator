#[allow(clippy::wildcard_imports)]
use leptos::*;

use crate::poker::Poker;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Poker simulator"</h1>
        <PokerHand/>
    }
}

fn PokerHand() -> impl IntoView {
    view! { <h1>"Poker Hand"</h1> }
}

pub fn mount_body() {
    let poker = Poker::new_texas_hold_em(4, 0);

    leptos::mount_to_body(App);
}
