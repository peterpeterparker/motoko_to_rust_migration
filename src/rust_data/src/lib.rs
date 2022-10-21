mod types;

use crate::types::{ Assets, State };
use ic_cdk::{ storage, print };
use ic_cdk_macros::{ init, query, update, pre_upgrade, post_upgrade };
use std::cell::RefCell;
use std::collections::HashMap;

// https://medium.com/encode-club/encode-x-internet-computer-intro-to-building-on-the-ic-in-rust-video-slides-b496d6baad08
// https://github.com/hpeebles/rust-canister-demo/blob/master/todo/src/lib.rs

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

// TODO: https://forum.dfinity.org/t/init-arg-mandatory-in-state/16009/ ?
// I would rather like to have a mandatory { owner: Principal } without having to assign a default value.

#[init]
fn init(user: String) {
    print(format!("Initializing bucket., {}", user));
    STATE.with(|state| {
        *state.borrow_mut() = State {
            owner: user,
            assets: HashMap::new(),
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    // TODO: Is there another a simpler way to save the all state?
    // STATE.with(|state| storage::stable_save((&state.borrow(),)).unwrap()); -> error

    let owner: String = STATE.with(|state| get_owner(&state.borrow()));
    let assets: Assets = STATE.with(|state| get_assets(&state.borrow()));

    let save: State = State { owner, assets };

    storage::stable_save((&save,)).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let (new_state,): (State,) = storage::stable_restore().unwrap();
    STATE.with(|state| {
        *state.borrow_mut() = new_state;
    });
}

#[query]
fn get() -> State {
    // TODO: redundant, same as in pre_upgrade
    let owner: String = STATE.with(|state| get_owner(&state.borrow()));
    let assets: Assets = STATE.with(|state| get_assets(&state.borrow()));

    let state: State = State { owner, assets };

    state
}

fn get_owner(state: &State) -> String {
    state.owner.to_string()
}

fn get_assets(state: &State) -> Assets {
    state.assets.clone()
}