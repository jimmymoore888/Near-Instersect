mod factory;
mod law;

// NOTE: OIM remains in the repo, but is intentionally NOT wired into the build yet.
// This keeps Factory v0 compilable while Module 8 is still being finalized.

use crate::factory::Factory;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

pub use law::{
    Airdrop, BurnCap, FixedSupply, LawV1Schema, LiquidityBootstrap, PercentageDistribution,
    TimeLock, VestingSchedule,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenRecordView {
    pub symbol: String,
    pub account: AccountId,
    pub created_by: AccountId,
    pub created_at_ns: u64,
    pub schema_hash: Base64VecU8,
    pub law: LawV1Schema,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    factory: Factory,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            factory: Factory::new(),
        }
    }

    /// Factory v0: register a token account under a symbol, bound to an immutable LAW schema.
    pub fn create_token(&mut self, symbol: String, account: AccountId, law: LawV1Schema) {
        self.factory.create_token(symbol.clone(), account.clone(), law);

        // Minimal event (NEP-297 style prefix, deterministic JSON)
        env::log_str(&format!(
            r#"EVENT_JSON:{{"standard":"near-intersect","version":"1.0.0","event":"TOKEN_REGISTERED","data":{{"symbol":"{}","account":"{}"}}}}"#,
            symbol.trim().to_uppercase(),
            account
        ));
    }

    pub fn get_token(&self, symbol: String) -> Option<AccountId> {
        self.factory.get_token(symbol)
    }

    pub fn get_record(&self, symbol: String) -> Option<TokenRecordView> {
        let sym = symbol.trim().to_uppercase();
        let r = self.factory.get_record(sym.clone())?;

        Some(TokenRecordView {
            symbol: sym,
            account: r.account,
            created_by: r.created_by,
            created_at_ns: r.created_at_ns,
            schema_hash: Base64VecU8(r.schema_hash),
            law: r.law,
        })
    }

    pub fn get_law(&self, symbol: String) -> Option<LawV1Schema> {
        self.get_record(symbol).map(|v| v.law)
    }
}
