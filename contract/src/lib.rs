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
    LiquidityPool,
    ElisionAccount,
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
    owener_id: AccountId,
    fee: u32,
    pool: LiquidityPool,
    locked_amount: u128,
    accounts: LookupMap<AccountId, ElisionAccount>,
    tokens: UnorderedSet<AccountId>,
    admins: UnorderedSet<AccountId>,
    settlement_address: AccountId,
    state: CurrentState
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// TODO: Write tests
}