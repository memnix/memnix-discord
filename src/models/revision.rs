use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixRevision {
    pub user_id: i8,
    pub card_id: i8,
    pub result: bool,
    pub result_int: i8,
}
