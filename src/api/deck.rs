use crate::models::{deck::MemnixDeck};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_deck(url: String) ->  Result<MemnixDeck> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut memnixaccess = MemnixDeck {
        id: 0,
        deck_name: "".to_string(),
    };

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixaccess = MemnixDeck {
            id: echo_json["data"]["ID"].to_string().parse::<u32>().unwrap(),
            deck_name: echo_json["data"]["deck_name"].to_string(),  
        };
    };

    Ok(memnixaccess)
}
