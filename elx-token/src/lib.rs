

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON: &str = "";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Team Token FT Tutorial".to_string(),
                symbol: "gtNEAR".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }
}