use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixUser {
    pub id: u32,
    pub user_name: String,
    pub discord_id: String,
    pub selected_deck: u32,
}
