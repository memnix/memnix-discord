use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixCard {
    pub id: u32,
    pub question: String,
    pub answer: String,
    pub image_url: String,
    pub card_type: String,
    //pub deck_id: i64,
    //pub tips: String,
    pub explication: String
    
}
