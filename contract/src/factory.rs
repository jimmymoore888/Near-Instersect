use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, AccountId};
use near_sdk::PanicOnDefault;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Factory {
    registry: UnorderedMap<String, AccountId>,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            registry: UnorderedMap::new(b"r".to_vec()),
        }
    }

    pub fn create_token(&mut self, symbol: String, account: AccountId) {
        let s = symbol.to_uppercase();
        assert!(!self.registry.contains_key(&s), "SYMBOL_USED");
        self.registry.insert(&s, &account);
        env::log_str(&format!("TokenCreated {} {}", s, account));
    }

    pub fn get_token(&self, symbol: String) -> Option<AccountId> {
        self.registry.get(&symbol.to_uppercase())
    }
}
