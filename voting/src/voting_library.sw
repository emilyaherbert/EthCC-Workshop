library voting_library;

use core::ops::Eq;

use std::contract_id::ContractId;

abi Voting {
    // initialize with the governance token
    #[storage(read, write)]
    fn initialize(token: ContractId);

    // get the amount of governance tokens in this contract
    #[storage(read)]
    fn get_balance() -> u64;

    #[storage(read)]
    fn get_user_balance() -> u64;

    // get the current favorite number
    #[storage(read)]
    fn get_favorite_number() -> u64;

    // get the current number of votes for a particular number
    #[storage(read)]
    fn get_number_of_votes(number: u64) -> u64;

    // the contract caller deposits governance tokens
    #[storage(read, write)]
    fn deposit();

    // withdraw governance tokens and send them to the user who called the contract
    #[storage(read, write)]
    fn withdraw(amount: u64);

    // vote for the new favorite number, where votes spend governance tokens
    #[storage(read, write)]
    fn vote(voting_for: u64, vote_amount: u64);

    // execute the votes and sets the favorite number to the number with the most votes
    //
    // returns true if a new favorite number is set, and returns false if one is not set (e.g. in a tie)
    #[storage(read, write)]
    fn execute() -> bool;
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
