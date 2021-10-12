use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixAccess {
    pub id: i8,
    pub user_id: i8,
    pub deck_id: i8,
    pub permission: i8,
    
}
