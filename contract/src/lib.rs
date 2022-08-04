/*
 * Elision options trading protocol that handles issuance, trading, and settlement of options.
 * 
 * Protocol reflects different changes in options premiums dynamically.
 *  - Collaterlization
 *  - Change in option price
 *  - Change in underlying price
 *  - Changer in time of maturity
 *  - volatility (change of change)
 *  - Interest Rates
 * 
 * Options are a way to hedge against volatility, reduce risk, or limit loss from price drops.
 * Learn more at https://elision.x
 * 
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};
use near_sdk::collections::LookupMap;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    LiquidityPools,
    ElisionAccounts,
    TokenList,
    Admin
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum CurrentState {
    Active,
    Paused
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    fee: u32,
    pools: Vector<LiquidityPool>,
    accounts: LookupMap<AccountId, ElisionAccount>,
    tokens: UnorderedSet<AccountId>,
    admins: UnorderedSet<AccountId>,
    state: CurrentState,
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: ValidAccountId, base_fee: u32) -> Self {
        Self {
            owner: owner.as_ref().clone(),
            state: CurrentState::Active,
            fee: base_fee,
            tokens: UnorderedSet::new(StorageKey::Tokenlist),
            admins: UnorderedSet::new(StorageKey::Admin),
            pools: Vector::new(StorageKey::LiquidityPools),
            accounts: LookupMap::new(StorageKey::ElisionAccounts),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// TODO: Write tests
}
