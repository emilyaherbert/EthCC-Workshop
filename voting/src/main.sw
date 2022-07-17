contract;

dep voting_library;
dep errors;

use voting_library::*;
use errors::*;

use std::{
    contract_id::ContractId,
    assert::require,
    storage::StorageMap,
    identity::Identity,
    context::{call_frames::msg_asset_id, msg_amount, this_balance},
    chain::auth::msg_sender,
    result::*,
    option::*,
    token::transfer,
};

storage {
    // tells us if it is initialized yet or not
    state: State = State::NotInitialized,

    // contract id of the governance token
    token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },

    // the max value that the favorite number could be
    max_num: u64 = 0,

    // the current favorite number
    favorite_number: u64 = 0,

    // the current number of votes for each prospective favorite number
    number_votes: StorageMap<u64, u64> = StorageMap {},

    // the balance for each user
    user_balances: StorageMap<Identity, u64> = StorageMap {},
}

impl Voting for Contract {
    // initialize with the governance token
    #[storage(read, write)]
    fn initialize(token: ContractId, max_num: u64) {
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.token = token;
        storage.max_num = max_num;
        storage.state = State::Initialized;

        let mut i = 0;
        while i < storage.max_num {
            storage.number_votes.insert(i, 0);
            i += 1;
        }
    }

    // get the amount of governance tokens in this contract
    #[storage(read)]
    fn get_balance() -> u64 {
        this_balance(storage.token)
    }

    // get the current favorite number
    #[storage(read)]
    fn get_favorite_number() -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        storage.favorite_number
    }

    // deposit governance tokens
    #[storage(read, write)]
    fn deposit() {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(storage.token == msg_asset_id(), UserError::IncorrectAssetSent);
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        let user = msg_sender().unwrap();
        storage.user_balances.insert(user, msg_amount() + storage.user_balances.get(user));
    }

    // withdraw governance tokens
    #[storage(read, write)]
    fn withdraw(amount: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(0 < amount, UserError::AmountCannotBeZero);

        let user: Identity = msg_sender().unwrap();
        let prev_balance = storage.user_balances.get(user);

        require(amount <= prev_balance, UserError::InsufficientBalance);

        storage.user_balances.insert(user, prev_balance - amount);

        // Transfer the asset back to the user
        transfer(amount, storage.token, user);
    }

    // vote for the new favorite number
    #[storage(read, write)]
    fn vote(voting_for: u64, vote_amount: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(voting_for < storage.max_num, UserError::InvalidNumber);

        let user = msg_sender().unwrap();
        let user_balance = storage.user_balances.get(user);

        require(vote_amount <= user_balance, UserError::InsufficientBalance);

        storage.user_balances.insert(user, user_balance - vote_amount);
        storage.number_votes.insert(voting_for, storage.number_votes.get(voting_for) + vote_amount);
    }

    // execute the votes
    #[storage(read, write)]
    fn execute() -> bool {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        let mut highest_votes = 0;
        let mut new_favorite_number = Option::None;
        let mut is_tie = false;

        let mut i = 0;
        while i < storage.max_num {
            let number_votes = storage.number_votes.get(i);
            if number_votes > highest_votes {
                highest_votes = number_votes;
                new_favorite_number = Option::Some(i);
                is_tie = false;
            } else if number_votes == highest_votes {
                is_tie = true;
            }
            i += 1;
        }

        match new_favorite_number {
            Option::Some(new_favorite_number) => {
                if !is_tie {
                    let mut i = 0;
                    while i < storage.max_num {
                        storage.number_votes.insert(i, 0);
                        i += 1;
                    }
                    return true;
                }
            },
            Option::None => {},
        }

        return false;
    }
}
