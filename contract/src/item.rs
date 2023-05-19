use crate::{Contract, ContractExt, Item, StorageKeys};
use near_sdk::store::Vector;
use near_sdk::{env, log, near_bindgen, AccountId, Promise};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_item(&mut self, title: String, description: String) {
        log!("{} posted {}!", env::predecessor_account_id(), &title);

        Promise::new(env::current_account_id()).transfer(env::attached_deposit());
        self.items.push(Item {
            seller: env::predecessor_account_id(),
            title: title,
            description: description,
            price: env::attached_deposit(),
        });
        let mut a: &mut Vector<u32> = match self
            .itemIdx_of_user
            .get_mut(&env::predecessor_account_id())
        {
            Some(T) => T,
            None => {
                self.itemIdx_of_user.insert(
                    env::predecessor_account_id(),
                    Vector::new(StorageKeys::SubAccount {
                        account_hash: env::sha256_array(env::predecessor_account_id().as_bytes()),
                    }),
                );
                self.itemIdx_of_user
                    .get_mut(&env::predecessor_account_id())
                    .unwrap()
            }
        };
        a.push(self.items.len() - 1);
    }

    pub fn get_item(&self, idx: u32) -> &Item {
        log!("get the {}th item", idx);
        self.items.get(idx).unwrap()
    }

    pub fn get_items_by_id(&self, id: AccountId) -> Vec<&Item> {
        match self.itemIdx_of_user.get(&id) {
            None => vec![],
            Some(T) => T.iter().map(|idx| self.items.get(*idx).unwrap()).collect(),
        }
    }

    pub fn get_posts(&self, mut from: u32, mut limit: u32) -> Vec<&Item> {
        let mut vec = vec![];
        while let Some(T) = self.items.get(from) {
            vec.push(T);
            from += 1;
            limit -= 1;
            if(limit==0){
                break;
            }
        }
        vec
    }
}
