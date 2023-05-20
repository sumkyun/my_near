use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
    pub seller: AccountId,
    pub title: String,
    pub description: String,
    pub price: Balance,
    pub item_id: ItemId,
    pub state: State,
    pub src: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    ONSALE,
    TRADING,
    SOLD,
}

#[near_bindgen]
impl Contract {
    #[payable]
    // 판매글 등록하기
    pub fn add_item(&mut self, title: String, description: String, src: String) {
        // 돈 보내기
        Promise::new(env::predecessor_account_id()).transfer(env::attached_deposit());

        let item_id=self.items.len();
        let seller=env::predecessor_account_id();

        // 판매글 등록
        self.items.push(Item {
            seller: seller.clone(),
            title: title,
            description: description,
            price: env::attached_deposit(),
            item_id: item_id,
            state: State::ONSALE,
            src: src,
        });

        // 유저정보에 판매글 번호 등록
        if !self.users.contains_key(&seller) {
            self.users.insert(
                seller.clone(),
                UserInfo::new(&seller),
            );
        }
        self.users
            .get_mut(&env::predecessor_account_id())
            .unwrap()
            .items_index
            .push(item_id);

        // 오더에 추가
        let mut prefix = Vec::with_capacity(33);
        prefix.push(b'w');
        prefix.extend(env::sha256(self.items.len().to_string().as_bytes()));
        self.orders.insert(self.items.len()-1, UnorderedSet::new(prefix));
    }

    // 판매글 가져오기
    pub fn get_item(&self, item_id:ItemId) -> &Item {
        self.items.get(item_id).unwrap()
    }

    // 아이디로 판매글 가져오기
    pub fn get_items_by_id(&self, seller: AccountId) -> Vec<&Item> {
        match self.users.get(&seller) {
            None => vec![],
            Some(T) => T
                .items_index
                .iter()
                .map(|idx| self.items.get(*idx).unwrap())
                .collect(),
        }
    }

    // 판매글 최신순으로 가져오기
    pub fn get_items(&self, mut from: ItemId, mut limit: ItemId) -> Vec<&Item> {
        let mut vec = vec![];
        let total = self.get_number_of_items();
        if total < from + 1 {
            return vec;
        }
        from = total - from - 1;
        while limit != 0 {
            vec.push(self.items.get(from).unwrap());
            if from == 0 {
                break;
            }
            from -= 1;
            limit -= 1;
        }
        vec
    }
}
