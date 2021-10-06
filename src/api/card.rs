use crate::models::card::MemnixCard;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_card(url: String) -> Result<MemnixCard> {
    let mut memnixcard = MemnixCard {
        question: "none".to_string(),
        answer: "none".to_string(),
        id: 0,
    };

    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixcard = MemnixCard {
            question: echo_json["data"]["card_question"].to_string(),
            answer: echo_json["data"]["card_answer"].to_string(),
            id: echo_json["data"]["ID"].to_string().parse::<i8>().unwrap(),
        };
    };

    Ok(memnixcard)
}
