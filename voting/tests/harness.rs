#![allow(dead_code)]

mod utils;

use fuels::prelude::*;
use utils::*;

#[cfg(test)]
mod should_pass {
    use super::*;

    #[tokio::test]
    async fn deployer_can_mint() {
        let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // expect user_1, user_2, and user_3 to ea&ch have no tokens
        let user_1_tokens =
            get_token_balance_in_wallet(token_contract_id, user_1.wallet.clone()).await;
        let user_2_tokens =
            get_token_balance_in_wallet(token_contract_id, user_2.wallet.clone()).await;
        let user_3_tokens =
            get_token_balance_in_wallet(token_contract_id, user_3.wallet.clone()).await;
        assert_eq!(user_1_tokens, None);
        assert_eq!(user_2_tokens, None);
        assert_eq!(user_3_tokens, None);

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 2_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 3_000_000, user_3.wallet.address()).await;

        // expect users to have tokens
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_1.wallet).await,
            Some(1_000_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_2.wallet).await,
            Some(2_000_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_3.wallet).await,
            Some(3_000_000)
        );
    }

    #[tokio::test]
    async fn deployer_can_initialize_the_voting_contract() {
        let (token_contract_id, _, [deployer, _, _, _]) = setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // check starting favorite number
        assert_eq!(get_favorite_number(&deployer.voting_handle).await, 0);
    }

    #[tokio::test]
    async fn users_can_deposit() {
        let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_3.wallet.address()).await;

        // deposit tokens from users
        deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 100_000).await;
        deposit_into_voting_contract(&user_2.voting_handle, token_contract_id, 200_000).await;
        deposit_into_voting_contract(&user_3.voting_handle, token_contract_id, 300_000).await;

        // expect users to have tokens
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_1.wallet).await,
            Some(900_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_2.wallet).await,
            Some(800_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_3.wallet).await,
            Some(700_000)
        );

        // expect the contract balance to have tokens
        assert_eq!(get_contract_balance(&deployer.voting_handle).await, 600_000);
    }

    #[tokio::test]
    async fn users_can_deposit_and_withdraw() {
        let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_3.wallet.address()).await;

        // deposit tokens from users
        deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_2.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_3.voting_handle, token_contract_id, 500_000).await;

        // withdraw tokens from the contract
        withdraw_from_voting_contract(&user_1.voting_handle, 100_000).await;
        withdraw_from_voting_contract(&user_2.voting_handle, 200_000).await;
        withdraw_from_voting_contract(&user_3.voting_handle, 300_000).await;

        // expect users to have tokens
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_1.wallet).await,
            Some(600_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_2.wallet).await,
            Some(700_000)
        );
        assert_eq!(
            get_token_balance_in_wallet(token_contract_id, user_3.wallet).await,
            Some(800_000)
        );

        // expect the contract balance to have tokens
        assert_eq!(get_contract_balance(&deployer.voting_handle).await, 900_000);
    }

    #[tokio::test]
    async fn users_can_vote() {
        let (token_contract_id, token_handle, [deployer, user_1, _, _]) = setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;

        // deposit tokens from users
        deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 500_000).await;

        // vote for numbers
        vote_for_number(&user_1.voting_handle, 5, 500_000).await;

        // check to see that the votes went through
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 5).await,
            500_000
        );

        // execute the voting system and check to see if a new favorite number was set
        assert!(execute_in_voting_contract(&deployer.voting_handle).await);

        // check starting favorite number
        assert_eq!(get_favorite_number(&deployer.voting_handle).await, 5);
    }

    #[tokio::test]
    async fn multiple_users_can_vote() {
        let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_3.wallet.address()).await;

        // deposit tokens from users
        deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_2.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_3.voting_handle, token_contract_id, 500_000).await;

        // vote for numbers
        vote_for_number(&user_1.voting_handle, 5, 100_000).await;
        vote_for_number(&user_2.voting_handle, 55, 200_000).await;
        vote_for_number(&user_3.voting_handle, 99, 200_001).await;

        // check to see that the votes went through
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 5).await,
            100_000
        );
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 55).await,
            200_000
        );
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 99).await,
            200_001
        );

        // execute the voting system and check to see if a new favorite number was set
        assert!(execute_in_voting_contract(&deployer.voting_handle).await);

        // check starting favorite number
        assert_eq!(get_favorite_number(&deployer.voting_handle).await, 99);
    }

    #[tokio::test]
    async fn users_can_vote_for_multiple_numbers() {
        let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) =
            setup_tests().await;

        // initialize the voting contract
        initialize_voting_contract(token_contract_id, &deployer.voting_handle).await;

        // mint tokens and send them to users
        mint_and_send_to_address(&token_handle, 1_000_000, user_1.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_2.wallet.address()).await;
        mint_and_send_to_address(&token_handle, 1_000_000, user_3.wallet.address()).await;

        // deposit tokens from users
        deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_2.voting_handle, token_contract_id, 500_000).await;
        deposit_into_voting_contract(&user_3.voting_handle, token_contract_id, 500_000).await;

        // vote for numbers
        vote_for_number(&user_1.voting_handle, 5, 100_000).await;
        vote_for_number(&user_1.voting_handle, 1, 400_000).await;
        vote_for_number(&user_2.voting_handle, 55, 200_000).await;
        vote_for_number(&user_2.voting_handle, 56, 1).await;
        vote_for_number(&user_2.voting_handle, 57, 1).await;
        vote_for_number(&user_3.voting_handle, 99, 200_001).await;
        vote_for_number(&user_3.voting_handle, 4, 200_001).await;

        // check to see that the votes went through
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 5).await,
            100_000
        );
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 1).await,
            400_000
        );
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 55).await,
            200_000
        );
        assert_eq!(get_number_of_votes(&deployer.voting_handle, 56).await, 1);
        assert_eq!(get_number_of_votes(&deployer.voting_handle, 57).await, 1);
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 99).await,
            200_001
        );
        assert_eq!(
            get_number_of_votes(&deployer.voting_handle, 4).await,
            200_001
        );

        // execute the voting system and check to see if a new favorite number was set
        assert!(execute_in_voting_contract(&deployer.voting_handle).await);

        // check starting favorite number
        assert_eq!(get_favorite_number(&deployer.voting_handle).await, 1);
    }
}
