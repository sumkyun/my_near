use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserInfo {
    pub items_index: Vector<u32>,
    pub trades_index: Vector<u32>,
    pub orders_index: UnorderedSet<u32>,
}

impl UserInfo {
    pub fn new(id: &AccountId) -> UserInfo {
        let mut prefix1 = Vec::with_capacity(33);
        prefix1.push(b'y');
        prefix1.extend(env::sha256(id.as_bytes()));
        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'z');
        prefix2.extend(env::sha256(id.as_bytes()));
        let mut prefix3 = Vec::with_capacity(33);
        prefix3.push(b'x');
        prefix3.extend(env::sha256(id.as_bytes()));
        
        UserInfo {
            items_index: Vector::new(prefix1),
            trades_index: Vector::new(prefix2),
            orders_index:UnorderedSet::new(prefix3),
        }
    }
}
