use near_units::parse_gas;
use near_units::parse_near;
use workspaces::prelude::*;
use workspaces::{Account, Contract, DevNetwork, Worker, result::ExecutionOutcome, InMemorySigner};
use near_sdk::{env, Promise, Gas, AccountId};
use near_crypto::{SecretKey, KeyType};

use near_primitives::views::FinalExecutionStatus;

const ACCESS_KEY_WASM_FILEPATH: &str = "../examples/access-key/res/access_key.wasm";

/*pub async fn init_status_message(
    worker: &workspaces::Worker<impl DevNetwork>,
) -> anyhow::Result<workspaces::Contract> {
    let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await?;
    /*let outcome = contract
        .call(worker, "default")
        .args_json(serde_json::json!({
        }))?
        .gas(parse_gas!("150 Tgas") as u64)
        .transact()
        .await?;
    match outcome.status {
        near_primitives::views::FinalExecutionStatus::SuccessValue(_) => (),
        _ => panic!("Failed to deploy"),
    };*/
    Ok(contract)
}*/

#[tokio::test]
async fn test_add_access_key_positive() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(ACCESS_KEY_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let owner = worker.root_account();
    /*let _user1 = owner
        .create_subaccount(&worker, "user1")
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await?
        .unwrap();*/

    let mut alice_string = String::from("alice.");
    alice_string.push_str(contract.id());
    let alice = AccountId::new_unchecked(alice_string);
    let mut bob_string = String::from("bob.");
    bob_string.push_str(contract.id());
    let bob = AccountId::new_unchecked(bob_string);

    let sk = SecretKey::from_seed(KeyType::ED25519, "seed_phrase");
    let pk = sk.public_key();
    owner
        .call(&worker, &contract.id(), "access_key")
        .args_json(serde_json::json!({
            "account_id": alice.clone(),
            "public_key": pk,
            "allowance": 10,
            "receiver_id": bob,
        }))?
        .transact()
        .await?;

    Ok(())
}