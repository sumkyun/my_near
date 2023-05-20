use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade {
    pub seller: AccountId,
    pub buyer: AccountId,
    pub item_id: ItemId,
    pub start: Timestamp,
    pub end: Timestamp,
    pub trade_id:TradeId,
}

#[near_bindgen]
impl Contract {
    pub fn add_trade(&mut self, item_id: ItemId) {
        self.trades.push(Trade {
            seller: self.items.get(item_id).unwrap().seller.clone(),
            buyer: env::predecessor_account_id(),
            item_id: item_id,
            start: env::block_timestamp_ms(),
            end: env::block_timestamp_ms()+259200000,
            trade_id: self.trades.len(),
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
