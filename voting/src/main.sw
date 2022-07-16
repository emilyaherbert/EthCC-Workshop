contract;

dep voting_library;
dep errors;

use voting_library::*;
use errors::*;

use std::contract_id::ContractId;
use std::assert::require;
use std::storage::StorageMap;
use std::identity::Identity;
use std::context::{call_frames::msg_asset_id, msg_amount, this_balance};
use std::chain::auth::msg_sender;
use std::result::*;
use std::token::transfer;

const MAX_NUM: u64 = 3;

storage {
    // tells us if it is initialized yet or not
    state: State = State::NotInitialized,

    // contract id of the governance token
    token: ContractId = ContractId {
        value: 0x0000000000000000000000000000000000000000000000000000000000000000,
    },

    // the current favorite number
    favorite_number: u64 = 0,

    // the current number of votes for each prospective favorite number
    number_votes: StorageMap<u64, u64> = StorageMap {},

    // the balance for each user
    user_balances: StorageMap<Identity, u64> = StorageMap {},

    // the number of votes for each user and prospective favorite number
    // that they voted for
    user_votes: StorageMap<(Identity, u64), u64> = StorageMap {},
}

impl Voting for Contract {
    // initialize with the governance token
    #[storage(read, write)]
    fn initialize(gov_token: ContractId) {
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        storage.token = gov_token;
        storage.state = State::Initialized;

        let mut i = 0;
        while i < MAX_NUM {
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

    /// get the amount of votes a user has used on a proposal
    #[storage(read)]
    fn get_user_votes(voting_for: u64) -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(voting_for < MAX_NUM, UserError::InvalidNumber);

        let user = msg_sender().unwrap();
        storage.user_votes.get((user, voting_for))
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
        require(voting_for < MAX_NUM, UserError::InvalidNumber);

        let user = msg_sender().unwrap();
        let user_balance = storage.user_balances.get(user);

        require(vote_amount <= user_balance, UserError::InsufficientBalance);

        let mut number_votes = storage.number_votes.get(voting_for);
        let mut user_votes = storage.user_votes.get((user, voting_for));
        number_votes += vote_amount;
        user_votes += vote_amount;

        storage.user_balances.insert(user, user_balance - vote_amount);
        storage.user_votes.insert((user, voting_for), user_votes);
        storage.number_votes.insert(voting_for, number_votes);
    }

    // unlock votes from a number and put them in the user balance
    #[storage(read, write)]
    fn unlock_votes(voting_for: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(voting_for < MAX_NUM, UserError::InvalidNumber);

        let user: Identity = msg_sender().unwrap();
        let user_votes = storage.user_votes.get((user, voting_for));

        storage.user_votes.insert((user, voting_for), 0);
        storage.user_balances.insert(user, storage.user_balances.get(user) + user_votes);
    }

    // execute the votes
    #[storage(read, write)]
    fn execute() {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        let mut highest_votes = 0;
        let mut new_favorite_number = storage.favorite_number;

        let mut i = 0;
        while i < MAX_NUM {
            let number_votes = storage.number_votes.get(i);
            if number_votes > highest_votes {
                highest_votes = number_votes;
                new_favorite_number = i;
            }
            
            i += 1;
        }

        storage.favorite_number = new_favorite_number;
    }
}
