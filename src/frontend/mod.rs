#[allow(clippy::wildcard_imports)]
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::{
    deck::Deck,
    poker::{player::Player, Poker},
};

#[component]
fn App() -> impl IntoView {
    let mut poker = Poker::new_texas_hold_em(4, 0);
    poker.shuffle();
    poker.deal_all_players();
    let (players, set_players) = create_signal(poker.board.players);
    view! {
        <div>
            <h1>{"Poker"}</h1>
            <PlayerHands players=(players, set_players)/>
        </div>
    }
}

#[component]
fn PlayerHands(players: (ReadSignal<Vec<Player>>, WriteSignal<Vec<Player>>)) -> impl IntoView {
    let (player_state, set_player_state) = players;
    let my_todos = move || {
        player_state
            .get()
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };
    view! {
        <ul class="todo-list">
            <For
                each=my_todos
                key=|todo_key| todo_key.0
                children=move |item| {
                    view! {
                        <li class="new-todo">
                            {item.1.get_hand_as_str()}
                        // <button class="remove" on:click=move |_| {}>
                        // {" Remove"}
                        // </button>
                        </li>
                    }
                }
            />

        </ul>
    }
}
// #[component]
// fn PokerTable(players: ReadSignal<Vec<Player>>) -> impl IntoView {
//     view! {}
// }

pub fn mount_body() {
    leptos::mount_to_body(App);
}

#[wasm_bindgen]
pub fn log_to_console(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

// <For each=players key=|state| state.id let:child>
//     <p>{child.hand[0].suit.to_string()} " " {child.hand[0].value.to_string()}</p>
// </For>
