use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixUser {
    pub id: i8,
    pub user_name: String,
    pub discord_id: String,
}
