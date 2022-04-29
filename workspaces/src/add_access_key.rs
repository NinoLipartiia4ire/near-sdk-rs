use near_units::parse_gas;
use near_units::parse_near;
use workspaces::prelude::*;
use workspaces::{Account, Contract, DevNetwork, Worker, result::ExecutionOutcome, InMemorySigner};
use near_sdk::{env, Promise, Gas, AccountId};
use near_crypto::{SecretKey, KeyType};

use near_primitives::views::FinalExecutionStatus;

const ACCESS_KEY_WASM_FILEPATH: &str = "../examples/access-key/res/access_key.wasm";

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
            "receiver_id": bob.clone(),
        }))?
        .transact()
        .await?;
    
    let mut credentials = String::from("~/.near-credentials/testnet/");
    credentials.push_str(contract.id().as_str());
    credentials.push_str(".json");
    //println!("{}", credentials);
    //let workspaces_alice = Account::from_file(credentials);
    

    Ok(())
}