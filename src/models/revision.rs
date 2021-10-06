use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixRevision {
    pub user_id: i64,
    pub card_id: i64,
    pub result: bool,
    
}
