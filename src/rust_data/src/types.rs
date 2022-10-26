use std::collections::HashMap;
use candid::{CandidType, Principal};
use serde::Deserialize;
use std::clone::Clone;

pub type Assets = HashMap<String, Asset>;

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct State {
    pub owner: Option<Principal>,
    pub assets: Assets,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Asset {
    key: String,
    value: Vec<u8>,
}