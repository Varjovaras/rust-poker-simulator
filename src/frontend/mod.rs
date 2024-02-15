#[allow(clippy::wildcard_imports)]
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::poker::{player::Player, Poker};

#[component]
fn App() -> impl IntoView {
    let poker = Poker::new_texas_hold_em(4, 0);
    let (poker_state, set_poker_state) = create_signal(poker);
    poker_signal_shuffle(set_poker_state);
    let (players, set_players) = create_signal(Vec::new());

    view! {
        <div>
            <h1>{"Poker"}</h1>
            <PokerTable players=(players, set_players)/>

            <button on:click=move |_| {
                deal_new_hand(set_poker_state, set_players);
                console_log("new hand");
            }>

                {"new hand"}
            </button>
        </div>
    }
}

fn deal_new_hand(set_poker_state: WriteSignal<Poker>, set_players: WriteSignal<Vec<Player>>) {
    set_poker_state.update(|poker| {
        poker.new_deck();
        poker.shuffle();
        poker.deal_all_players();
        console_log(poker.board.players[0].get_hand_as_str().as_str());
        set_players.set(poker.board.players.clone());
    });
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
pub fn console_log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

fn poker_signal_shuffle(set_poker_state: WriteSignal<Poker>) {
    set_poker_state.update(|poker| {
        poker.shuffle();
        poker.deal_all_players();
    });
}

// <For each=players key=|state| state.id let:child>
//     <p>{child.hand[0].suit.to_string()} " " {child.hand[0].value.to_string()}</p>
// </For>
