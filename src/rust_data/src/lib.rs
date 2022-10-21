mod types;

use crate::types::{ Assets, State, Asset };
use ic_cdk::{ storage, print, api };
use ic_cdk_macros::{ init, query, update, pre_upgrade, post_upgrade };
use std::cell::RefCell;
use std::collections::HashMap;
use candid::{CandidType, decode_args};
use serde::Deserialize;

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

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

#[derive(CandidType, Deserialize)]
struct UpgradeState {
    pub user: Option<UpgradeUser>,
    pub entries: Option<Vec<(String, Asset)>>,
}

#[derive(CandidType, Deserialize)]
struct UpgradeUser {
    user: String,
}

#[pre_upgrade]
fn pre_upgrade() {
    // TODO: Is there another a simpler way to save the all state?
    // STATE.with(|state| storage::stable_save((&state.borrow(),)).unwrap()); -> error

    // let owner: String = STATE.with(|state| get_owner(&state.borrow()));
    // let assets: Assets = STATE.with(|state| get_assets(&state.borrow()));

    // let entries: Vec<(String, Asset)> = assets.into_iter().collect();

    // let user: UpgradeUser = UpgradeUser {user: owner};
    // let save: UpgradeState = UpgradeState { user, entries };

    // storage::stable_save((&save,)).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    // TODO: error Panicked at 'called `Result::unwrap()` on an `Err` value: "Custom(Cannot parse header
    // Issue: https://forum.dfinity.org/t/upgrade-canister-from-motoko-to-rust-with-stable-memory/
    
    // The solution is to go through candid but, it seems controversial. Still looking for an answer.

    // Post upgrade from Rust to Rust

    /* let (upgrade_state,): (UpgradeState,) = storage::stable_restore().unwrap();

    let assets: Assets = upgrade_state.entries.into_iter().collect();

    let new_state: State = State {
        owner: upgrade_state.user,
        assets,
    };

    STATE.with(|state| {
        *state.borrow_mut() = new_state;
    }); */

    // Post upgrade from Motoko to Rust

    // https://docs.rs/ic-cdk/0.3.2/src/ic_cdk/api/stable.rs.html#64
    // let bytes: Vec<u8> = api::stable::stable_bytes();

    // By senior.joinu - not all heroes wear capes
    let mut stable_length_buf = [0u8; std::mem::size_of::<u32>()];
    api::stable::stable_read(0, &mut stable_length_buf);
    let stable_length = u32::from_le_bytes(stable_length_buf); // maybe use from_be_bytes, I don't remember what endianess is candid
 
    let mut buf = vec![0u8; stable_length as usize];
    api::stable::stable_read(std::mem::size_of::<u32>() as u32, &mut buf);

    // let size = (api::stable::stable_size() as usize) << 16;
    // let mut vec = Vec::with_capacity(size);
    // unsafe {
    //     vec.set_len(size);
    // }
    // api::stable::stable_read(std::mem::size_of::<u32>() as u32, vec.as_mut_slice());

    let (upgrade_state,): (UpgradeState,) = decode_args(&buf).unwrap();
    
    let assets: Assets = upgrade_state.entries.unwrap().into_iter().collect();

    let new_state: State = State {
        owner: upgrade_state.user.unwrap().user,
        assets,
    };

    STATE.with(|state| {
        *state.borrow_mut() = new_state;
    });
}

// From dfn_candid https://github.com/dfinity/ic/blob/89446f5a04f053040b4863eab5458446d925ed0e/rs/rust_canisters/dfn_candid/src/lib.rs
// fn from_bytes(bytes: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), String> {
//    let res = decode_args(&bytes).map_err(|e| e.to_string())?;
//    Ok(res)
// }

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