use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::Serialize;
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTContract {
    owner_by_id: UnorderedMap<u128, AccountId>,
    token_id: u128,
    token_by_id: UnorderedMap<u128, Token>,
}

impl Default for NFTContract {
    fn default() -> Self {
        Self {
            owner_by_id: UnorderedMap::new(b'm'),
            token_id: 0,
            token_by_id: UnorderedMap::new(b'n')
        }
    }
}
