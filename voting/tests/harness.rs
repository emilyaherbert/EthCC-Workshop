use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(Voting, "./out/debug/voting-abi.json");
abigen!(MyToken, "../token/out/debug/token-abi.json");

struct User {
    voting_handle: Voting,
    wallet: LocalWallet,
}

#[tokio::test]
async fn voting_test() {
    let (token_contract_id, token_handle, [deployer, user_1, user_2, user_3]) = setup_tests().await;

    // // initialize the voting dao
    // deployer
    //     .voting_handle
    //     .initialize(token_contract_id, 3)
    //     .call()
    //     .await
    //     .unwrap();

    // println!("{:?}", user_1.voting_handle.get_balance());

    // // we expect the favorite number to start at 0
    // assert_eq!(
    //     deployer
    //         .voting_handle
    //         .get_favorite_number()
    //         .call()
    //         .await
    //         .unwrap()
    //         .value,
    //     0
    // );
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
