# Voting Contract

This contract will handle voting for the group's favorite numbers.

In order to best showcase the features of Sway during this workshop, we will be using test-driving-development using the Rust SDK. The Rust SDK is an SDK for Fuel that allows you to compile, deploy, and test Sway contracts, launch a local Fuel network, and more, in Rust. In this example, the Rust SDK was used to create [some initial testing](tests/harness.rs) that should help guide your implementation of the Sway contract.

The skeleton of the contract is written for you already, but you will be completing the body of it.

### Steps

1. Familiarize yourself with [the ABI](src/voting_library.sw) for the contract we will be writing

2. Familiarize yourself with [the error types](src/errors.sw) that we will be using in our contract 

2. Ensure that you are inside this subdirectory in your terminal, then run:

    ```bash
    $ forc test
    failures:
        should_pass::deployer_can_initialize_the_voting_contract
        should_pass::deployer_can_mint
        should_pass::multiple_users_can_vote
        should_pass::users_can_deposit
        should_pass::users_can_deposit_and_withdraw
        should_pass::users_can_vote
        should_pass::users_can_vote_for_multiple_numbers

    test result: FAILED. 0 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.32s
    ```

    We expect all the test to fail at this point! We will be filling in the contract functions to make these tests pass.

3. Complete the `initialize` function in [the `src/main.sw` file](src/main.sw)

4. Ensure that you are inside this subdirectory in your terminal, then run:

    ```bash
    $ forc test
    Compiled library "core".
    Compiled library "std".
    Compiled contract "token".
    Bytecode size is 84 bytes.
    ```
