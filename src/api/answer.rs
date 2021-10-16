use crate::models::{answer::MemnixAnswer};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_answer(url: String) ->  Result<MemnixAnswer> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut memnixanswer = MemnixAnswer {
        card_id: 0,
        answer: "none".to_string(),
    };

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixanswer = MemnixAnswer {
            card_id: echo_json["data"]["card_id"].to_string().parse::<u32>().unwrap(),
            answer: echo_json["data"]["answer"].to_string(),
        };
    };

    Ok(memnixanswer)
}