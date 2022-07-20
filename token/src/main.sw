contract;

use std::{
    address::Address,
    context::call_frames::contract_id,
    token::mint_to_address
};

abi MyToken {
    fn mint_and_send_to_address(amount: u64, recipient: Address) -> bool;
}

impl MyToken for Contract {
    fn mint_and_send_to_address(amount: u64, recipient: Address) -> bool {
        // TODO:
        //
        // mint tokens and send them to an address using the "mint_to_address" function
        //
        // the function signature for mint_to_address:
        // fn mint_to_address(amount: u64, to: Address)

        true
    }
}
