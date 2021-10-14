use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixRevision {
    pub user_id: u32,
    pub card_id: u32,
    pub result: bool,
    pub result_int: i8,
    pub quality: i8
}
