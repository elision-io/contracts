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

    // List of existing options
    minted_options: Vec<ElisionOption>,

    // Size of NEAR in the pool.
    amounts: Vec<Balance>,

    // Shares of the pool by liquidity providers.
    shares: LookupMap<AccountId, Balance>,

    // Total number of shares
    total_shares: Balance,

    // Fee charges by the pool, 0-100
    pool_fee: u32,

    // Collateral needed to be locked when option is purchased, 0-100
    collaterilzation_ratio: u32,

    // Option positions from the pool
    positions: LiquidityPosition,

    // Minter account address
    minter_address: AccountId
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

    #[payable]
    pub fn sell_option(
        buyer: AccountId,
        duration: u32,
        amount: U128,
        strike: U128
    ) {
        // If strike price is 0, set strike price to currentPrice()
        
        // Method: total_balance() -> Balance
        // Method: calc_lock_amount() -> Balance

        // If duration is less that or equal to 1 day - "Duration to short"
        // If duration is grater than or equal to 90 days - "Duration is too long"
        
        // `(locked_amount + amount_to_lock) * 100 <= amount * max_utilization_rate` - Amount is too large
        
        // Method: calc_total_premium() -> (settlement_fee, premium)

        // let hedged_premium_total: U128 = (premium * hedged_balance) / balance;
        // let hedge_fee: U128 = (hedged_premium_total * hedge_fee_rate) / 100;
        // let hedge_premium: U128 = hedged_premium_total - hedge_fee;
        // let unhedge_premium: U128 = premium - hedged_premium_total;

        let locked_amount += amount_to_lock;
        id = ElisionOption::ext create_option(buyer);
        options[id] = Option(
            option_type: OptionType::Call,
            option_state: OptionState::Open,
            owner: buyer,
            amount: amount,
            locked_amount: amount_to_lock,
            creation: env::block_timestamp,
            expiration: env::block_timestamp + duration,
            pool_address: env::current_account_id,
            settlement_address: env::current_account_id,
            strike: strike,
            premium: OptionPricer::calc_total_premium(duration, amount, strike)
        )

        // locked_amount
        ElisionOption::ext(self.minter_address.clone())
        .with_static_gas(5*TGAS)
        .create_option(buyer)
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .transfer_option(id: TokenId, receiver_id: buyer)
        )        
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};
}