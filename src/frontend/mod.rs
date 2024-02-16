#[allow(clippy::wildcard_imports)]
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::poker::{player::Player, Poker};

#[component]
fn App() -> impl IntoView {
    let (poker_signal, set_poker_signal) = create_signal(Poker::new_texas_hold_em(4, 0));
    let mut poker = poker_signal.get();
    poker.shuffle(); // Change the return type of shuffle to `()`
    set_poker_signal(poker);
    poker.deal_all_players();
    set_poker_signal(poker);
    let (players, set_players) = create_signal(poker.board.players.clone());
    view! {
        <div>
            <h1>{"Poker"}</h1>
            <PlayerHands players=(players, set_players)/>
            <button on:click=move |_| {
                poker.new_deck_and_shuffle();
                poker.deal_all_players();
                poker = Poker::new_texas_hold_em(4, 0);
                set_players(poker.board.players.clone());
                log_to_console("deal");
                log_to_console(&poker.board.players[0].get_hand_as_str());
            }>

                {"Deal"}
            </button>
        </div>
    }
}

#[component]
fn PlayerHands(players: (ReadSignal<Vec<Player>>, WriteSignal<Vec<Player>>)) -> impl IntoView {
    let (player_state, _set_player_state) = players;
    // let my_todos = move || {
    //     player_state
    //         .get()
    //         .iter()
    //         .map(|item| (item.id, item.clone()))
    //         .collect::<Vec<_>>()
    // };
    view! {
        <ul class="todo-list">
            <For
                each=player_state
                key=|player| player.id
                children=move |item| {
                    view! {
                        <li class="new-todo">
                            // {"shuffle"}
                            {item.get_hand_as_str()}
                            <button class="remove" on:click=move |_| {}></button>
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
