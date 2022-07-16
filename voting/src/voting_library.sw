library voting_library;

use core::ops::Eq;

use std::contract_id::ContractId;

abi Voting {
    #[storage(read, write)]
    fn initialize(gov_token: ContractId);

    #[storage(read)]
    fn get_balance() -> u64;

    #[storage(read)]
    fn get_favorite_number() -> u64;

    #[storage(read)]
    fn get_user_votes(voting_for: u64) -> u64;

    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn withdraw(amount: u64);

    #[storage(read, write)]
    fn vote(voting_for: u64, vote_amount: u64);

    #[storage(read, write)]
    fn unlock_votes(voting_for: u64);

    #[storage(read, write)]
    fn execute();
}

pub enum State {
    NotInitialized: (),
    Initialized: (),
}

impl Eq for State {
    fn eq(self, other: Self) -> bool {
        match(self, other) {
            (State::Initialized, State::Initialized) => true,
            (State::NotInitialized, State::NotInitialized) => true,
            _ => false, 
        }
    }
}
