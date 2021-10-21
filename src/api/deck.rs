use crate::models::{deck::MemnixDeck};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_deck(url: String) ->  Result<MemnixDeck> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut memnixdeck = MemnixDeck {
        id: 0,
        deck_name: "".to_string(),
        status: 0,
    };

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixdeck = MemnixDeck {
            id: echo_json["data"]["ID"].to_string().parse::<u32>().unwrap(),
            deck_name: echo_json["data"]["deck_name"].to_string(),
            status: echo_json["data"]["status"].to_string().parse::<u32>().unwrap(),
        };
    };

    Ok(memnixdeck)
}

pub async fn fetch_decks(url: String) -> Result<Vec<MemnixDeck>> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let mut array: Vec<MemnixDeck> = Vec::new();

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        for x in 0..echo_json["count"].to_string().parse::<u32>().unwrap() {
            let memnixdeck = MemnixDeck {
                id: echo_json["data"][x.to_string().parse::<usize>().unwrap()]["ID"].to_string().parse::<u32>().unwrap(),
                deck_name: echo_json["data"][x.to_string().parse::<usize>().unwrap()]["deck_name"].to_string(),
                status: echo_json["data"][x.to_string().parse::<usize>().unwrap()]["status"].to_string().parse::<u32>().unwrap(),
            };
            array.push(memnixdeck);
        }
    };
    Ok(array)
}

pub async fn post_deck(url: String, deck: MemnixDeck) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&deck)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
