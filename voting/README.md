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
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... FAILED
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... FAILED
    test should_pass::users_can_vote_and_execute ... FAILED
    test should_pass::users_can_deposit_and_withdraw ... FAILED
    test should_pass::multiple_users_can_vote_and_execute ... FAILED
    test should_pass::multiple_users_can_vote ... FAILED
    test should_pass::users_can_deposit ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... FAILED

    ...

    test result: FAILED. 1 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s
    ```

    `forc test` runs `forc build` under the hood and then executes the Rust tests (using the Fuel Rust SDK) that test our contract 😊 The yellow warnings that you see are compiler warnings alerting you that parts of your contract are going unused and can be removed (this is expected because we are still filling stuff in). If you want to see this in action, you can run `forc build` and see that the compiler output is the same as `forc test`---just minus the tests.

    We expect most of the tests to fail at this point. We will be filling in the contract functions to make these tests pass. The only test that passes right now is the `deployer_can_mint`, this passes because of work that we did on the Token contract in the previous section.

3. Complete the `initialize` function in [the `src/main.sw` file](src/main.sw)

4. Run:

    ```bash
    $ forc test
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... ok
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... FAILED
    test should_pass::users_can_vote_and_execute ... FAILED
    test should_pass::users_can_deposit_and_withdraw ... FAILED
    test should_pass::multiple_users_can_vote_and_execute ... FAILED
    test should_pass::multiple_users_can_vote ... FAILED
    test should_pass::users_can_deposit ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... FAILED

    ...

    test result: FAILED. 2 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.03s
    ```

    You should see that an additional test `deployer_can_initialize_the_voting_contract` passes.

5. Complete the `deposit` function.

6. Run:

    ```bash
    $ forc test
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... ok
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... FAILED
    test should_pass::users_can_vote_and_execute ... FAILED
    test should_pass::users_can_deposit_and_withdraw ... FAILED
    test should_pass::multiple_users_can_vote_and_execute ... FAILED
    test should_pass::multiple_users_can_vote ... FAILED
    test should_pass::users_can_deposit ... ok
    test should_pass::users_can_vote_for_multiple_numbers ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... FAILED

    ...

    test result: FAILED. 3 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.97s
    ```

    You should see that an additional test `users_can_deposit` passes.

7. Complete the `withdraw` function.

6. Run:

    ```bash
    $ forc test
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... ok
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... FAILED
    test should_pass::users_can_vote_and_execute ... FAILED
    test should_pass::multiple_users_can_vote_and_execute ... FAILED
    test should_pass::multiple_users_can_vote ... FAILED
    test should_pass::users_can_deposit ... ok
    test should_pass::users_can_deposit_and_withdraw ... ok
    test should_pass::users_can_vote_for_multiple_numbers ... FAILED
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... FAILED

    ...

    test result: FAILED. 4 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.71s
    ```

    You should see that an additional test `users_can_deposit_and_withdraw` passes.

8. Complete the `vote` function.

9. Run:

    ```bash
    $ forc test
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... ok
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... ok
    test should_pass::users_can_vote_and_execute ... FAILED
    test should_pass::users_can_deposit ... ok
    test should_pass::multiple_users_can_vote ... ok
    test should_pass::multiple_users_can_vote_and_execute ... FAILED
    test should_pass::users_can_deposit_and_withdraw ... ok
    test should_pass::users_can_vote_for_multiple_numbers ... ok
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... FAILED

    ...

    test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.54s
    ```

    You should see that three additional tests pass, `users_can_vote`, `multiple_users_can_vote`, and `users_can_vote_for_multiple_numbers`.

10. Complete the `execute` function.

11. Run:

    ```bash
    $ forc test
    ...

    running 10 tests
    test should_pass::deployer_can_initialize_the_voting_contract ... ok
    test should_pass::deployer_can_mint ... ok
    test should_pass::users_can_vote ... ok
    test should_pass::users_can_vote_and_execute ... ok
    test should_pass::users_can_deposit ... ok
    test should_pass::multiple_users_can_vote ... ok
    test should_pass::users_can_deposit_and_withdraw ... ok
    test should_pass::multiple_users_can_vote_and_execute ... ok
    test should_pass::users_can_vote_for_multiple_numbers ... ok
    test should_pass::users_can_vote_for_multiple_numbers_and_execute ... ok

    ...

    test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.54s
    ```

    You should see that three additional tests pass, `users_can_vote_and_execute`, `multiple_users_can_vote_and_execute`, and `users_can_vote_for_multiple_numbers_and_execute`.

