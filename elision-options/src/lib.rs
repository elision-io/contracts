use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue, Timestamp
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;

// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[derive(BorshSerialize)]
pub enum OptionState {
    Inactive,
    Active,
    Exercised,
    Expired
}

#[derive(BorshSerialize)]
pub enum OptionType {
    Call,
    Put,
    Invalid
}

#[derive(BorshSerialize)]
pub enum PositionState {
    Invalid,
    Open,
    Closed
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ElisionOption {
    // Name of option containing basic info
    name: String,

    // Contains more verbose contract information
    description: String,

    // Call or Put
    option_type: OptionType,

    // Status of the option contract
    option_state: OptionState,

    // Account address that's holding the contract, usually the buyer
    owner: AccountId,

    // Account address of the liquidity pool that sold and backs the underlying assets
    pool_address: AccountId,

    // Stable token address
    stable_address: AccountId,

    // Underlying assets token address
    asset_address: AccountId,

    // Account of the recipient of the 1% settlement fee
    settlement_address: AccountId,

    // Price that asset can be bought / sold at within option duration
    strike: U128,

    // Fee paid by the buyer for to buy / sell asset at a specified price
    premium: U128,

    // Size of the options contract e.g. 100 NEAR
    amount: Balance,

    // When the option was created
    creation: Timestamp,
    
    // When the option will expire
    expiration: Timestamp
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LiquidityPosition {
    // Owner account of liquidity position
    owner: AccountId,

    // Current state of position: Invalid, Open, Closed
    state: PositionState,

    // Liquidity provider's share in the pool
    share: U128,

    // Size of the liquidity position
    amount: Balance,

    // When the position was create
    creation: Timestamp
}

impl Default for LiquidityPosition {
    fn default() -> Self {
        Self {
            owner: env::current_account_id(),
            state: PositionState::Open,
            share: 0,
            amount: 0,
            creation: env::block_timestamp()
        }
    }
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExternalContract {
    register_address: AccountId,
    contract_name: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //keeps track of whitelisted contracts
    pub whitelist_contracts: LookupMap<AccountId, ExternalContract>

}

// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    ContractAllowed
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Elision Option Token".to_string(),
                symbol: "OELX".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}