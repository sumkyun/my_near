use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub seller: AccountId,
    pub buyer: AccountId,
    pub item_idx: u32,
    pub start: Timestamp,
    pub end: Timestamp,
}

#[near_bindgen]
impl Contract {
    pub fn add_trade(&mut self, item_index: u32) {
        self.trades.push(Trade {
            seller: self.items.get(item_index).unwrap().seller.clone(),
            buyer: env::predecessor_account_id(),
            item_idx: item_index,
            start: env::block_timestamp(),
            end: env::block_timestamp()+259200000000,
        })
        
    }

    pub fn get_trades_by_id(&self, id: AccountId) -> Vec<&Trade> {
        match self.users.get(&id) {
            None => vec![],
            Some(T) => T
                .trades_index
                .iter()
                .map(|idx| self.trades.get(*idx).unwrap())
                .collect(),
        }
    }
}
