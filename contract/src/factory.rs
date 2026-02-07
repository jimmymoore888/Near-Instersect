use near_sdk::{near_bindgen, env, AccountId, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Factory {
    registry: UnorderedMap<String, AccountId>,
}

#[near_bindgen]
impl Factory {
    #[init]
    pub fn new() -> Self {
        Self {
            registry: UnorderedMap::new(b"r".to_vec()),
        }
    }

    pub fn create_token(&mut self, symbol: String, account: AccountId) {
        let s = symbol.to_uppercase();
        self.registry.insert(&s, &account);
        env::log_str(&format!("TokenCreated {}", s));
    }

    pub fn get_token(&self, symbol: String) -> Option<AccountId> {
        self.registry.get(&symbol.to_uppercase())
    }
}