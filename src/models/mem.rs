use serde::{Deserialize, Serialize};

use super::card::MemnixCard;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemnixMem {
    pub id: u32,
    //pub user_id: i8,
    //pub card_id: i8,
    //pub deck_id: i8,
    pub quality: i8,
    pub repetition: i8,
    pub efactor: f32,
    pub interval: i8,
    pub total: i8,
    pub card: MemnixCard,
    // pub next_date: str

}
