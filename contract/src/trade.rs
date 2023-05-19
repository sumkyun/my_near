use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Trade{
    pub seller:AccountId,
    pub buyer:AccountId,
    pub item_idx:u32,
    pub start:Timestamp,
    pub end:Timestamp,
}