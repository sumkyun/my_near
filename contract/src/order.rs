use crate::*;

#[near_bindgen]
impl Contract {
    pub fn buyer_order(&mut self, item_id: ItemId) {
        let seller = self.items.get(item_id.clone()).unwrap().seller.clone();
        let buyer = env::predecessor_account_id();

        self.orders.get_mut(&item_id).unwrap().insert(buyer.clone());
        if !self.users.contains_key(&buyer) {
            self.users.insert(buyer, UserInfo::new(&buyer));
        }
        self.users
            .get_mut(&buyer)
            .unwrap()
            .orders_index
            .insert(item_id);
    }

    pub fn seller_order(&mut self, item_id: ItemId, buyer: AccountId) {
        let seller = env::predecessor_account_id();

        for buyer in self.orders.get_mut(&item_id).unwrap().iter() {
            self.users
                .get_mut(buyer)
                .unwrap()
                .orders_index
                .remove(&item_id);
        }
        self.orders.get_mut(&item_id).unwrap().clear();

        let trade_id: TradeId = self.trades.len();
        self.trades.push(Trade {
            seller: seller.clone(),
            buyer: buyer.clone(),
            item_id: item_id,
            start: env::block_timestamp_ms(),
            end: env::block_timestamp_ms() + 5 * 24 * 3600 * 1000,
            trade_id: trade_id.clone(),
        });

        self.users
            .get_mut(&seller)
            .unwrap()
            .trades_index
            .push(trade_id.clone());
        self.users
            .get_mut(&buyer)
            .unwrap()
            .trades_index
            .push(trade_id.clone());
    }

    pub fn is_on_buy_order(&self, buyer: AccountId, item_id: ItemId) -> bool {
        self.orders
            .get(&item_id)
            .unwrap()
            .contains(&buyer)
    }
}
