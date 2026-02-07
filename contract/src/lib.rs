mod factory;

use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;

pub use factory::Factory;

#[near_bindgen]
#[derive(PanicOnDefault)]
pub struct Contract {
    total_supply: Balance,
    circulating_supply: Balance,
    locked_supply: Balance,
    genesis_ts: u64,
    interval_ns: u64,
    balances: LookupMap<AccountId, Balance>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner: AccountId,
        total_supply: U128,
        locked_supply: U128,
        genesis_ts: u64,
        interval_ns: u64,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        let total: Balance = total_supply.into();
        let locked: Balance = locked_supply.into();
        assert!(locked <= total, "Locked exceeds total");

        let circulating = total - locked;

        let mut balances = LookupMap::new(b"b");
        balances.insert(&owner, &circulating);

        Self {
            total_supply: total,
            circulating_supply: circulating,
            locked_supply: locked,
            genesis_ts,
            interval_ns,
            balances,
        }
    }

    pub fn total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    pub fn balance_of(&self, account: AccountId) -> U128 {
        self.balances.get(&account).unwrap_or(0).into()
    }

    pub fn unlockable(&self) -> U128 {
        let now = env::block_timestamp();
        if now < self.genesis_ts {
            return 0.into();
        }
        let cycles = (now - self.genesis_ts) / self.interval_ns;
        let per_cycle = self.locked_supply / 1; // Phase-0 single-cycle full unlock
        (cycles as Balance * per_cycle).min(self.locked_supply).into()
    }
}