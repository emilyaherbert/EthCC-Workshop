use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Voting, "./out/debug/voting-abi.json");
abigen!(MyToken, "../token/out/debug/token-abi.json");

struct User {
    voting_handle: Voting,
    wallet: LocalWallet,
}

#[tokio::test]
async fn mint_test() {
    let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) = setup_tests().await;
    
    // initialize the voting contract
    initialize_voting_contract(token_contract_id, &deployer.voting_handle, 3).await;

    // expect user_1, user_2, and user_3 to each have no tokens
    let user_1_tokens = get_token_balance_in_wallet(token_contract_id, user_1.wallet.clone()).await;
    let user_2_tokens = get_token_balance_in_wallet(token_contract_id, user_2.wallet.clone()).await;
    let user_3_tokens = get_token_balance_in_wallet(token_contract_id, user_3.wallet.clone()).await;
    assert_eq!(user_1_tokens, None);
    assert_eq!(user_2_tokens, None);
    assert_eq!(user_3_tokens, None);

    // mint 10 tokens and send them to user_1
    mint_and_send_to_address(&token_handle, 10, user_1.wallet.address()).await;

    // mint 20 tokens and send them to user_2
    mint_and_send_to_address(&token_handle, 20, user_2.wallet.address()).await;

    // mint 30 tokens and send them to user_3
    mint_and_send_to_address(&token_handle, 30, user_3.wallet.address()).await;

    // expect user_1 to have 10 tokens
    assert_eq!(
        get_token_balance_in_wallet(token_contract_id, user_1.wallet).await,
        Some(10)
    );

    // expect user_2 to have 20 tokens
    assert_eq!(
        get_token_balance_in_wallet(token_contract_id, user_2.wallet).await,
        Some(20)
    );

    // expect user_3 to have 30 tokens
    assert_eq!(
        get_token_balance_in_wallet(token_contract_id, user_3.wallet).await,
        Some(30)
    );
}

#[tokio::test]
async fn deposit_test() {
    let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) = setup_tests().await;
    
    // initialize the voting contract
    initialize_voting_contract(token_contract_id, &deployer.voting_handle, 3).await;

    // mint tokens
    mint_and_send_to_address(&token_handle, 100, user_1.wallet.address()).await;
    mint_and_send_to_address(&token_handle, 100, user_2.wallet.address()).await;
    mint_and_send_to_address(&token_handle, 100, user_3.wallet.address()).await;

    println!("{:?}", user_1.voting_handle.get_balance().call().await.unwrap().receipts.last().unwrap().gas_used().unwrap());

    // deposit 10 tokens from user_1
    deposit_into_voting_contract(&user_1.voting_handle, token_contract_id, 10).await;

    // // deposit 10 tokens from user_1
    // deposit_into_voting_contract(&user_2.voting_handle, token_contract_id, 20).await;

    // // deposit 10 tokens from user_1
    // deposit_into_voting_contract(&user_3.voting_handle, token_contract_id, 30).await;

    // // expect user_1 to have 90 tokens
    // assert_eq!(
    //     get_token_balance_in_wallet(token_contract_id, user_1.wallet).await,
    //     Some(90)
    // );

    // // expect user_2 to have 80 tokens
    // assert_eq!(
    //     get_token_balance_in_wallet(token_contract_id, user_2.wallet).await,
    //     Some(80)
    // );

    // // expect user_3 to have 70 tokens
    // assert_eq!(
    //     get_token_balance_in_wallet(token_contract_id, user_3.wallet).await,
    //     Some(70)
    // );
}

async fn initialize_voting_contract(
    token_contract_id: ContractId,
    voting_handle: &Voting,
    max_num: u64,
) {
    voting_handle
        .initialize(token_contract_id, max_num)
        .call()
        .await
        .unwrap();
}

async fn get_contract_balance(voting_handle: Voting) -> u64 {
    voting_handle.get_balance().call().await.unwrap().value
}

async fn mint_and_send_to_address(token_handle: &MyToken, asset_amount: u64, address: Address) {
    token_handle
        .mint_and_send_to_address(asset_amount, address)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

async fn deposit_into_voting_contract(voting_handle: &Voting, token_contract_id: ContractId, asset_amount: u64) {
    let tx_params = TxParameters::new(
        None,                                           // gas price
        Some(10_000_000),                               // gas limit
        None,                                           // byte price
        None                                            // maturity
    );
    let call_params = CallParameters::new(
        Some(asset_amount),                             // amount
        Some(AssetId::from(*token_contract_id)),        // asset ID
        Some(1_000_000),                                // gas forwarded
    );
    voting_handle
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap();
}

async fn get_token_balance_in_wallet(
    token_contract_id: ContractId,
    wallet: LocalWallet,
) -> Option<u64> {
    let mut x_string = "0x".to_string();
    x_string.push_str(&token_contract_id.to_string());
    let balances = wallet
        .get_balances()
        .await
        .unwrap();
    // println!("--");
    // for (k, v) in balances.clone().into_iter() {
    //     println!("{:?}, {:?}", k, v);
    // }
    balances.get(&x_string).cloned()
}

async fn setup_tests() -> (ContractId, MyToken, [User; 4]) {
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
