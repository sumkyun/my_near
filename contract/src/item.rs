use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
    pub seller: AccountId,
    pub title: String,
    pub description: String,
    pub price: Balance,
    pub views: u32,
    pub scraps: u32,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_item(&mut self, title: String, description: String) {
        let preId=env::predecessor_account_id();
        let curId=env::current_account_id();
        let deposit=env::attached_deposit();

        Promise::new(curId).transfer(deposit);

        self.get_user_info_by_id(&preId).items.push(self.items.len());

        self.items.push(Item {
            seller: preId,
            title: title,
            description: description,
            price: deposit,
            views:0,
            scraps:0,
        });
    }

    pub fn get_item(&self, idx: u32) -> &Item {
        self.items.get(idx).unwrap()
    }

    pub fn get_items_by_id(&self, id: AccountId) -> Vec<&Item> {
        match self.users.get(&id) {
            None => vec![],
            Some(T) => T.items.iter().map(|idx| self.items.get(*idx).unwrap()).collect(),
        }
    }


    pub fn get_posts(&self, mut from: u32, mut limit: u32) -> Vec<&Item> {
        let mut vec = vec![];
        from=self.get_number_of_items()-from-1;
        while from>=0 && limit>0{
            vec.push(self.items.get(from).unwrap())
        }
        vec
    }
}
