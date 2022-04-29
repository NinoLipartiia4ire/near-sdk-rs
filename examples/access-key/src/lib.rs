use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, Promise, PublicKey};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AccessKey {}

#[near_bindgen]
impl AccessKey {
    pub fn full_access_key(&self, account_id: AccountId, public_key: PublicKey) -> Promise {
        Promise::new(account_id)
            .create_account()
            .transfer(2000000000000000000000000)
            .deploy_contract(
                include_bytes!("../../status-message/res/status_message.wasm").to_vec(),
            )
            .add_full_access_key(public_key)
    }

    pub fn access_key(
        &self,
        account_id: AccountId,
        public_key: PublicKey,
        allowance: Balance,
        receiver_id: AccountId,
    ) -> Promise {
        let p1 =
            Promise::new(receiver_id.clone()).create_account().transfer(2000000000000000000000000);
        let p2 = Promise::new(account_id)
            .create_account()
            .transfer(2000000000000000000000000)
            .deploy_contract(
                include_bytes!("../../status-message/res/status_message.wasm").to_vec(),
            )
            .add_access_key(public_key, allowance, receiver_id, "set_status".to_string());

        p1.then(p2)
    }
}
