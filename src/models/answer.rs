use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixAnswer {
    pub card_id: u32,
    pub answer: String,
    
}
