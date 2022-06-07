// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::LookupMap;

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
   
}

impl Default for Welcome {
  fn default() -> Self {
    Self {
      
    }
  }
}

#[near_bindgen]
impl Welcome {
   pub fn hello_world( $self)->String{
       "Hello World from Rust Smart Contract".to_String()
   }
}


