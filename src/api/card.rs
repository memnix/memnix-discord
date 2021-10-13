use crate::models::{card::MemnixCard, mem::MemnixMem};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/*
pub async fn fetch_card(url: String) -> Result<MemnixCard> {
    let mut memnixcard = MemnixCard {
        question: "none".to_string(),
        answer: "none".to_string(),
        id: 0,
    };

    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixcard = MemnixCard {
            question: echo_json["data"]["Card"]["card_question"].to_string(),
            answer: echo_json["data"]["Card"]["card_answer"].to_string(),
            id: echo_json["data"]["Card"]["ID"].to_string().parse::<i8>().unwrap(),
        };
    };

    Ok(memnixcard)
}
*/

pub async fn fetch_mem(url: String) -> Result<MemnixMem> {
    let mut memnixmem: MemnixMem = MemnixMem {
        id: 0,
        quality: 0,
        repetition: 0,
        efactor: 0.0,
        interval: 0,
        total:0,
        card: MemnixCard {
            question: "none".to_string(),
            answer: "none".to_string(),
            id: 0,
        }
    };

    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {

        memnixmem = MemnixMem {
            id: echo_json["data"]["ID"].to_string().parse::<i8>().unwrap(),
            quality: echo_json["data"]["quality"].to_string().parse::<i8>().unwrap(),
            repetition: echo_json["data"]["repetition"].to_string().parse::<i8>().unwrap(),
            efactor:echo_json["data"]["e_factor"].to_string().parse::<f32>().unwrap(),
            interval: echo_json["data"]["interval"].to_string().parse::<i8>().unwrap(),
            total: echo_json["data"]["total"].to_string().parse::<i8>().unwrap(),
            card: MemnixCard {
                question: echo_json["data"]["Card"]["card_question"].to_string(),
                answer: echo_json["data"]["Card"]["card_answer"].to_string(),
                id: echo_json["data"]["Card"]["ID"].to_string().parse::<i8>().unwrap(),
            },
        };
    };

    Ok(memnixmem)
}
