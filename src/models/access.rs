use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixAccess {
    pub id: u32,
    pub user_id: u32,
    pub deck_id: u32,
    pub permission: i8,
    
}
