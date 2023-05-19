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
    pub index: u32,
}

#[near_bindgen]
impl Contract {
    #[payable]
    // 판매글 등록하기
    pub fn add_item(&mut self, title: String, description: String) {
        // 돈 보내기
        Promise::new(env::predecessor_account_id()).transfer(env::attached_deposit());

        // 판매글 등록
        self.items.push(Item {
            seller: env::predecessor_account_id(),
            title: title,
            description: description,
            price: env::attached_deposit(),
            views: 0,
            scraps: 0,
            index: self.items.len(),
        });

        // 유저정보에 판매글 번호 등록
        self.add_item_index(env::predecessor_account_id(), self.items.len() - 1);
    }

    // 판매글 가져오기
    pub fn get_item(&mut self, idx: u32) -> &Item {
        let item = self.items.get_mut(idx).unwrap();
        // 조회수 1 증가
        item.views += 1;
        item
    }

    // 아이디로 판매글 가져오기
    pub fn get_items_by_id(&self, id: AccountId) -> Vec<&Item> {
        match self.users.get(&id) {
            None => vec![],
            Some(T) => T
                .items_index
                .iter()
                .map(|idx| self.items.get(*idx).unwrap())
                .collect(),
        }
    }

    // 판매글 최신순으로 가져오기
    pub fn get_posts(&self, mut from: u32, mut limit: u32) -> Vec<&Item> {
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

    pub fn scrap_item(&mut self, idx: u32) {
        // 판매글의 스크랩 수 증가
        self.items.get_mut(idx).unwrap().scraps += 1;
        // 유저정보에 스크랩 판매글 추가
        self.add_scrap_index(env::predecessor_account_id(), idx);
    }
}
