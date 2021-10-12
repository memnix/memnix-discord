use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixCard {
    pub user_id: i8,
    pub deck_id: i8,
    pub permission: i8,
    
}
