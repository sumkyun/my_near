use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserInfo {
    pub items_index: Vector<u32>,
    pub trades_index: Vector<u32>,
    pub scraps_index: UnorderedSet<u32>,
}

impl UserInfo {
    pub fn new(id: &AccountId) -> UserInfo {
        let mut prefix1 = Vec::with_capacity(33);
        prefix1.push(b'x');
        prefix1.extend(env::sha256(id.as_bytes()));
        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'y');
        prefix2.extend(env::sha256(id.as_bytes()));
        let mut prefix3 = Vec::with_capacity(33);
        prefix3.push(b'z');
        prefix3.extend(env::sha256(id.as_bytes()));
        UserInfo {
            items_index: Vector::new(prefix1),
            trades_index: Vector::new(prefix2),
            scraps_index: UnorderedSet::new(prefix3),
        }
    }
}

impl Contract {
    // 아이디로 유저 정보 가져오기
    pub fn get_mut_user_info_by_id(&mut self, id: AccountId) -> &mut UserInfo {
        if self.users.contains_key(&id) {
            self.users.get_mut(&id).unwrap()
        } else {
            self.users.insert(id.clone(), UserInfo::new(&id));
            self.users.get_mut(&id).unwrap()
        }
    }

    // 판매글 인덱스를 유저정보에 추가
    pub fn add_item_index(&mut self, id: AccountId, idx: u32) {
        self.get_mut_user_info_by_id(id).items_index.push(idx);
    }

    // 트레이드 인덱스를 유저정보에 추가
    pub fn add_trade_index(&mut self, id1: AccountId, id2: AccountId, idx: u32) {
        self.get_mut_user_info_by_id(id1).trades_index.push(idx);
        self.get_mut_user_info_by_id(id2).trades_index.push(idx);
    }

    // 스크랩 인덱스를 유저정보에 추가
    pub fn add_scrap_index(&mut self, id: AccountId, idx: u32) {
        self.get_mut_user_info_by_id(id).scraps_index.insert(idx);
    }
}
