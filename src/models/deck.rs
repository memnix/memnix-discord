use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixDeck {
    pub id: u32,
    pub deck_name: String,
    pub status: u32,

}
