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

const MAX_NUM = 100;

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
}

impl Voting for Contract {
    // initialize with the governance token
    #[storage(read, write)]
    fn initialize(token: ContractId) {
        require(storage.state == State::NotInitialized, InitializationError::CannotReinitialize);

        // TODO:
        //
        // 1. set the storage.token to the token
        // 2. set the storage.state to State::Initialized
        // 3. insert values into storage.number_votes from 0 to MAX_NUM using a while loop.
        //    set the initial number of votes for each to 0
    }

    // get the amount of governance tokens in this contract
    #[storage(read)]
    fn get_balance() -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        this_balance(storage.token) // this gets the balance of a particular coin (storage.token) for the current contract
    }

    // get the number of tokens a user has deposited in the contract
    #[storage(read)]
    fn get_user_balance() -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        let user = msg_sender().unwrap();
        storage.user_balances.get(user)
    }

    // get the current favorite number
    #[storage(read)]
    fn get_favorite_number() -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        storage.favorite_number
    }

    // get the current number of votes for a particular number
    #[storage(read)]
    fn get_number_of_votes(number: u64) -> u64 {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);
        require(number < MAX_NUM, UserError::InvalidNumber);
        storage.number_votes.get(number)
    }

    // the contract caller deposits governance tokens
    #[storage(read, write)]
    fn deposit() {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        // TODO:
        //
        // 1. get the Identity of the user that called the contract with the function call "msg_sender()".
        //    "msg_sender()" returns a Result, so we will need to call "msg_sender().unwrap()" to unwrap the
        //    Identity from inside of the Result
        // 2. get the amount of the deposit with the function call "msg_amount()"
        // 3. get the previous balance of the user from storage.user_balances
        // 4. calculate the new balance using the values from step (2) and step (3)
        // 5. update the user balance using storage.user_balances.insert and the value from step (4)
        //
        // NOTE:
        //
        // 1. consider the case in which the user attempts to deposit tokens that are not the governance token.
        //    in this case, use a require statement and return UserError::IncorrectAssetSent
        // 2. consider the case in which the user attemps to deposit 0 tokens. in this case, use a require
        //    statement and return UserError::AmountCannotBeZero
    }

    // withdraw governance tokens and send them to the user who called the contract
    #[storage(read, write)]
    fn withdraw(amount: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        // TODO:
        //
        // 1. get the Identity of the user that called the contract with the function call "msg_sender()".
        //    "msg_sender()" returns a Result, so we will need to call "msg_sender().unwrap()" to unwrap the
        //    Identity from inside of the Result
        // 2. get the previous balance of the user from storage.user_balances
        // 3. calculate the new balance using the value from step (2) and the amount to be withdrawn
        // 4. update the user balance using storage.user_balances.insert and the value from step (4)
        // 5. tranfer the asset back to the user using the "transfer" function
        //
        // function signature for the "transfer" function:
        // fn transfer(amount: u64, asset_id: ContractId, to: Identity)
        //
        // NOTE:
        //
        // 1. consider the case in which the amount to be withdrawn is less than 0. in this case, use a require
        //    statement and return UserError::AmountCannotBeZero
        // 2. consider the case in which the amount to be withdraw is greater then the previous balance from
        //    step (2). in this case, use a require statement and return UserError::InsufficientBalance
    }

    // vote for the new favorite number, where votes spend governance tokens
    #[storage(read, write)]
    fn vote(voting_for: u64, vote_amount: u64) {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        // TODO:
        //
        // 1. get the Identity of the user that called the contract with the function call "msg_sender()".
        //    "msg_sender()" returns a Result, so we will need to call "msg_sender().unwrap()" to unwrap the
        //    Identity from inside of the Result
        // 2. get the previous balance of the user from storage.user_balances
        // 3. calculate the new user balance using the value from step (2) and the "vote_amount" passed into the function
        // 4. update the user balance using storage.user_balances.insert and the value from step (3)
        // 5. get the current number of votes for the "voting_for" number from storage.number_votes
        // 6. calculate the new number of votes for "voting_for" using the "vote_amount" passed into the function
        // 7. update the number of votes for "voting_for" to the value from step (7)
        //
        // NOTE:
        //
        // 1. consider the case in which the number the user is voting for is greater than MAX_NUM. in this case,
        //    use a require statement and return UserError::InvalidNumber
        // 2. consider the case in which the number of votes the user wants to allocate is greater than their
        //    current balance. in this case, use a require statement and return UserError::InsufficientBalance
    }

    // execute the votes and sets the favorite number to the number with the most votes
    //
    // returns true if a new favorite number is set, and returns false if one is not set (e.g. in a tie)
    #[storage(read, write)]
    fn execute() -> bool {
        require(storage.state == State::Initialized, InitializationError::ContractNotInitialized);

        // TODO:
        //
        // 1. iterate through storage.number_votes using a while loop and determine the number with
        //    the highest number of votes, if there is any such number
        // 2. set the favorite number to the value from step (1)
        // 3. return true if a new favorite number is set, and false otherwise
        //
        // NOTE:
        //
        // 1. consider the case in which there are no votes at all, or when there is a tie in votes

        return false;
    }
}
