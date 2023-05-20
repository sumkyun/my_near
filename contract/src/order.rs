use crate::*;

#[near_bindgen]
impl Contract {
    pub fn buyer_order(&mut self, item_id: ItemId) -> bool {
        let seller=self.items.get(item_id.clone()).unwrap().seller.clone();
        let buyer=env::predecessor_account_id();
        assert!(seller!=buyer);


        if self.orders.get(&item_id).unwrap().contains(&(seller.clone(), buyer.clone())){
            // 판매자의 판매 주문이 존재함
            // 거래를 만들고 해당 아이템의 주문을 삭제함
            let trade_id=self.trades.len();
            self.trades.push(Trade { seller: seller.clone(), buyer: buyer.clone(), item_id: item_id, start: env::block_timestamp_ms(), end: env::block_timestamp_ms()+ 5*24*3600*1000,trade_id:trade_id,});
            self.orders.get_mut(&item_id).unwrap().clear();
            
            if !self.users.contains_key(&seller){
                self.users.insert(seller.clone(), UserInfo::new(&seller));
            }
            self.users.get_mut(&seller).unwrap().trades_index.push(trade_id.clone());
            self.users.get_mut(&buyer).unwrap().trades_index.push(trade_id.clone());
            true
        }
        else{
            // 판매자의 판매 주문이 존재함
            // 구매자의 구매 주문을 등록
            self.orders.get_mut(&item_id).unwrap().insert((buyer, seller));
            false
        }
    }

    pub fn seller_order(&mut self, item_id: ItemId, buyer:AccountId) -> bool {
        let seller=env::predecessor_account_id();
        assert!(seller!=buyer);


        if self.orders.get(&item_id).unwrap().contains(&(buyer.clone(), seller.clone())){
            // 구매자의 구매 주문이 존재함
            // 거래를 만들고 해당 아이템의 주문을 삭제함
            let trade_id=self.trades.len();
            self.trades.push(Trade { seller: seller.clone(), buyer: buyer.clone(), item_id: item_id, start: env::block_timestamp_ms(), end: env::block_timestamp_ms()+ 5*24*3600*1000,trade_id:trade_id,});
            self.orders.get_mut(&item_id).unwrap().clear();
            
            if !self.users.contains_key(&seller){
                self.users.insert(seller.clone(), UserInfo::new(&seller));
            }
            self.users.get_mut(&seller).unwrap().trades_index.push(trade_id.clone());
            self.users.get_mut(&buyer).unwrap().trades_index.push(trade_id.clone());
            true
        }
        else{
            // 구매자의 구매 주문이 존재함
            // 판매자의 판매 주문을 등록
            self.orders.get_mut(&item_id).unwrap().insert((seller, buyer));
            false
        }
    }

    pub fn is_on_sell_order(&self, seller:AccountId, buyer:AccountId, item_id: ItemId)->bool{
        self.orders.get(&item_id).unwrap().contains(&(seller, buyer))
    }

    pub fn is_on_buy_order(&self, buyer:AccountId, seller:AccountId, item_id: ItemId)->bool{
        self.orders.get(&item_id).unwrap().contains(&(buyer,seller))
    }

}
