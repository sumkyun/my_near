use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserInfo{
    pub items:Vector<u32>,
    pub trades:Vector<u32>,
    pub scraps:UnorderedSet<u32>,
}

impl UserInfo{
    pub fn new(id:&AccountId)->UserInfo{
        let mut prefix1 = Vec::with_capacity(33);
        prefix1.push(b'x');
        prefix1.extend(env::sha256(id.as_bytes()));
        let mut prefix2 = Vec::with_capacity(33);
        prefix2.push(b'y');
        prefix2.extend(env::sha256(id.as_bytes()));
        let mut prefix3 = Vec::with_capacity(33);
        prefix3.push(b'z');
        prefix3.extend(env::sha256(id.as_bytes()));
        UserInfo{
            items:Vector::new(prefix1),
            trades:Vector::new(prefix2),
            scraps:UnorderedSet::new(prefix3),
        }
    }
}

#[near_bindgen]
impl Contract{
    pub fn get_user_info_by_id(&mut self, id:&AccountId)->&UserInfo{
        let mut users=&self.users;
        match users.get(id){
            Some(T) => T,
            None => {users.insert(id.clone(), UserInfo::new(id));
            users.get(id).unwrap()},
        }
    }
}