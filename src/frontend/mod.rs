#[allow(clippy::wildcard_imports)]
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::poker::{player::Player, Poker};

#[component]
fn App() -> impl IntoView {
    let poker = Poker::new_texas_hold_em(4, 0);
    let (poker_state, set_poker_state) = create_signal(poker);

    set_poker_state.update(|poker| {
        poker.shuffle();
        poker.deal_all_players();
    });

    let (players, set_players) = create_signal(poker_state.get().board.players);
    view! {
        <div>
            <h1>{"Poker"}</h1>
            <PokerTable players=(players, set_players)/>

            <button on:click=move |_| {
                let mut new_players = players.get();
                new_players.push(Player::new(5));
                set_players.set(new_players);
            }>

                {"new hand"}
            </button>
        </div>
    }
}

#[component]
fn PokerTable(players: (ReadSignal<Vec<Player>>, WriteSignal<Vec<Player>>)) -> impl IntoView {
    let (players_state, set_players_state) = players;
    let players = move || {
        players_state
            .get()
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };
    view! {
        <ul class="todo-list">
            <For
                each=players
                key=|todo_key| todo_key.0
                children=move |player| {
                    view! {
                        <li class="new-todo">
                            <p>{player.1.name.clone()}</p>
                            {player.1.get_hand_as_str()}
                            <button on:click=move |_| {
                                let mut new_players = players_state.get();
                                new_players.retain(|x| x.id != player.0);
                                set_players_state.set(new_players);
                            }>{"fold"}</button>
                        </li>
                    }
                }
            />

        </ul>
    }
}

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
