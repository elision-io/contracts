use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Gas};
use near_sdk::json_types::ValidAccountId;

// Value that the fee will be divided by
pub const FEE_DIVISOR: u32 = 10_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OptionPool {
    // List of tokens in the pool
    token_addresses: Vec<AccountId>,

    // Size of NEAR in the pool.
    amounts: Vec<Balance>,

    // Shares of the pool by liquidity providers.
    shares: LookupMap<AccountId, Balance>,

    // Total number of shares
    total_shares: Balance,

    // Fee charges by the pool, 0-100
    pool_fee: u32,

    // Collateral needed to be locked when option is purchased, 0-100
    collaterilzation_ratio: u32

    // Option positions from the pool
    positions: LiquidityPosition
}

#[near_bindgen]
impl OptionPool {
    #[init]
    pub fn new(
        id: u32,
        token_addresses: Vec<ValidAccountId>,
        pool_fee: u32
    ) {
        Self {
            token_addresses: token_addresses.iter().map(|a| a.clone().into()).collect(),
            amounts: vec![0u128; 2],
            shares: LookupMap::new(StorageKey::Shares {
                pool_id: id,
            }),
            total_shares: 0,
            pool_fee,
            collaterilzation_ratio: 20,
            position: LiquidityPosition::default()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};
}