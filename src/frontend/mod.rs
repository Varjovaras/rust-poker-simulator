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
    // let (players, set_players) = create_signal(Vec::new());
    let players = move || poker_state().board.players;

    view! {
        <div>
            <h1>{"Poker"}</h1>
            <PokerTable players=Signal::derive(players) set_poker_state=set_poker_state/>

        // <button on:click=move |_| {
        // deal_new_hand(set_poker_state);
        // console_log("new hand");
        // }>

        // {"new hand"}
        // </button>
        </div>
    }
}

fn deal_new_hand(set_poker_state: WriteSignal<Poker>) {
    set_poker_state.update(|poker| {
        poker.new_deck();
        poker.shuffle();
        poker.deal_all_players();
        console_log(poker.board.players[0].get_hand_as_str().as_str());
    });
}

#[component]
fn PokerTable(
    #[prop(into)] players: Signal<Vec<Player>>,
    set_poker_state: WriteSignal<Poker>,
) -> impl IntoView {
    // let players = players.get();

    view! {
        <ul class="todo-list">
            <For
                each=players
                key=|todo_key| todo_key.id
                children=move |player| {
                    view! {
                        <li class="new-todo">
                            <p>{player.name.clone()}</p>
                            {player.get_hand_as_str()}
                            <button on:click=move |_| {}>{"fold"}</button>
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
