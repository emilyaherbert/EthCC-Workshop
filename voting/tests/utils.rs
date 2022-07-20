#![allow(dead_code)]

use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Voting, "./out/debug/voting-abi.json");
abigen!(MyToken, "../token/out/debug/token-abi.json");

pub(crate) struct User {
    pub(crate) voting_handle: Voting,
    pub(crate) wallet: LocalWallet,
}

pub(crate) async fn setup_tests() -> (ContractId, MyToken, [User; 4]) {
    let num_wallets = 4;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;
    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );
    let mut wallets = launch_custom_provider_and_get_wallets(config, None).await;
    let deployer_wallet = wallets.pop().unwrap();
    let user_1_wallet = wallets.pop().unwrap();
    let user_2_wallet = wallets.pop().unwrap();
    let user_3_wallet = wallets.pop().unwrap();

    let token_contract_id = Contract::deploy(
        "../token/out/debug/token.bin",
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../token/out/debug/token-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let token_handle = MyToken::new(token_contract_id.to_string(), deployer_wallet.clone());

    let voting_contract_id = Contract::deploy(
        "./out/debug/voting.bin",
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/voting-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let deployer = User {
        voting_handle: Voting::new(voting_contract_id.to_string(), deployer_wallet.clone()),
        wallet: deployer_wallet,
    };
    let user_1 = User {
        voting_handle: Voting::new(voting_contract_id.to_string(), user_1_wallet.clone()),
        wallet: user_1_wallet,
    };
    let user_2 = User {
        voting_handle: Voting::new(voting_contract_id.to_string(), user_2_wallet.clone()),
        wallet: user_2_wallet,
    };
    let user_3 = User {
        voting_handle: Voting::new(voting_contract_id.to_string(), user_3_wallet.clone()),
        wallet: user_3_wallet,
    };

    (
        token_contract_id,
        token_handle,
        [deployer, user_1, user_2, user_3],
    )
}

pub(crate) async fn initialize_voting_contract(
    token_contract_id: ContractId,
    voting_handle: &Voting,
) {
    voting_handle
        .initialize(token_contract_id)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn mint_and_send_to_address(
    token_handle: &MyToken,
    asset_amount: u64,
    address: Address,
) {
    token_handle
        .mint_and_send_to_address(asset_amount, address)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn deposit_into_voting_contract(
    voting_handle: &Voting,
    token_contract_id: ContractId,
    asset_amount: u64,
) {
    let tx_params = TxParameters::new(
        None,             // gas price
        Some(10_000_000), // gas limit
        None,             // byte price
        None,             // maturity
    );
    let call_params = CallParameters::new(
        Some(asset_amount),                      // amount
        Some(AssetId::from(*token_contract_id)), // asset ID
        Some(1_000_000),                         // gas forwarded
    );
    voting_handle
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn withdraw_from_voting_contract(voting_handle: &Voting, asset_amount: u64) {
    voting_handle
        .withdraw(asset_amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn vote_for_number(voting_handle: &Voting, voting_for: u64, vote_amount: u64) {
    voting_handle
        .vote(voting_for, vote_amount)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn execute_in_voting_contract(voting_handle: &Voting) -> bool {
    voting_handle
        .execute()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn get_contract_balance(voting_handle: &Voting) -> u64 {
    voting_handle.get_balance().call().await.unwrap().value
}

pub(crate) async fn get_user_balance(voting_handle: &Voting) -> u64 {
    voting_handle
        .get_user_balance()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn get_token_balance_in_wallet(
    token_contract_id: ContractId,
    wallet: LocalWallet,
) -> Option<u64> {
    let mut x_string = "0x".to_string();
    x_string.push_str(&token_contract_id.to_string());
    let balances = wallet.get_balances().await.unwrap();
    // println!("--");
    // for (k, v) in balances.clone().into_iter() {
    //     println!("{:?}, {:?}", k, v);
    // }
    balances.get(&x_string).cloned()
}

pub(crate) async fn get_favorite_number(voting_handle: &Voting) -> u64 {
    voting_handle
        .get_favorite_number()
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn get_number_of_votes(voting_handle: &Voting, number: u64) -> u64 {
    voting_handle
        .get_number_of_votes(number)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}
