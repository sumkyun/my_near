use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, CryptoHash, Timestamp};

mod item;
mod trade;
mod user;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub items: Vector<Item>,
    pub item_index_of_user: UnorderedMap<AccountId, Vector<u32>>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
    pub seller: AccountId,
    pub title: String,
    pub description: String,
    pub price: Balance,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade{
    pub seller:AccountId,
    pub buyer:AccountId,
    pub item_idx:u32,
    pub start:Timestamp,
    pub end:Timestamp,
}

pub struct UserInfo{

}

impl Default for Contract {
    fn default() -> Self {
        Self {
            items: Vector::new(b"p".to_vec()),
            item_index_of_user: UnorderedMap::new(StorageKeys::Accounts),
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Accounts,
    SubAccount { account_hash: CryptoHash },
}

#[near_bindgen]
impl Contract {
    pub fn get_number_of_users(&self) -> u32 {
        self.item_index_of_user.len()
    }
    pub fn get_number_of_items(&self) -> u32 {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::Balance;

    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn write_post_and_get_post() {
        let mut contract = Contract::default();

        set_context("tester1.testnet", 1 * NEAR);

        assert_eq!(contract.get_number_of_items(), 0);
        contract.add_item("apple".to_string(), "amazing".to_string());
        assert_eq!(contract.get_number_of_items(), 1);
        let a = contract.get_item(0);

        println!("{}", near_sdk::serde_json::to_string(a).unwrap());

        assert_eq!(env::predecessor_account_id(), a.seller);
    }

    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }
}
