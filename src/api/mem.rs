use crate::models::{card::MemnixCard, mem::MemnixMem};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/*
pub async fn post_mem(url: String, mem: MemnixMem) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&mem)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
*/

pub async fn fetch_mem(url: String) -> Result<MemnixMem> {
    let mut memnixmem: MemnixMem = MemnixMem {
        quality: 0,
        repetition: 0,
        efactor: 0.0,
        interval: 0,
        total:0,
        card: MemnixCard {
            question: "none".to_string(),
            answer: "none".to_string(),
            id: 0,
            image_url: "none".to_string(),
            card_type: "none".to_string(),
        }
    };

    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {

        memnixmem = MemnixMem {
            quality: echo_json["data"]["quality"].to_string().parse::<i8>().unwrap(),
            repetition: echo_json["data"]["repetition"].to_string().parse::<i8>().unwrap(),
            efactor:echo_json["data"]["e_factor"].to_string().parse::<f32>().unwrap(),
            interval: echo_json["data"]["interval"].to_string().parse::<i8>().unwrap(),
            total: echo_json["data"]["total"].to_string().parse::<i8>().unwrap(),
            card: MemnixCard {
                question: echo_json["data"]["Card"]["card_question"].to_string(),
                answer: echo_json["data"]["Card"]["card_answer"].to_string(),
                id: echo_json["data"]["Card"]["ID"].to_string().parse::<u32>().unwrap(),
                image_url: echo_json["data"]["Card"]["image_url"].to_string(),
                card_type: echo_json["data"]["Card"]["card_type"].to_string(),
            },
        };
    };

    Ok(memnixmem)
}
