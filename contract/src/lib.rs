use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use near_sdk::serde::Serialize;


#[derive(BorshSerialize,BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Token {
    token_id: u128,
    owner_id: AccountId,
    name: String,
    description: String,
    media_url: String,
    level: u128,
}