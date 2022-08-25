use near_sdk::{ext_contract};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

// Interface of this contract, for callbacks
#[ext_contract(elision_pool)]
trait Callbacks {
  fn create_option_callback(&mut self) -> String;
  fn transfer_option_callback(&mut self) -> bool;
}

// Validator interface, for cross-contract calls
#[ext_contract(elision_options)]
trait ElisionOption {
  #[payable]
  fn create_option(&mut self) -> ElisionOption;
  fn transfer_option(&mut self, receiver_id: AccountId)
}