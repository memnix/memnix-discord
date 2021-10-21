use crate::models::user::MemnixUser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_user_id(url: String) -> Result<u32> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: u32;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<u32>().unwrap();
    } else {
        id = 0;
    }

    Ok(id)
}

pub async fn fetch_user(url: String) -> Result<MemnixUser> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut user = MemnixUser {
        id: 0,
        user_name: "none".to_string(),
        discord_id: "none".to_string(),
        selected_deck: 0,
    };
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        user.id = echo_json["data"]["ID"].to_string().parse::<u32>().unwrap();
        user.user_name = echo_json["data"]["user_name"].to_string().replace("\"", "");
        user.discord_id = echo_json["data"]["discord_id"].to_string().replace("\"", "");
        user.selected_deck = echo_json["data"]["selected_deck"].to_string().parse::<u32>().unwrap();
    };

    Ok(user)
}

/*
pub async fn fetch_selected_deck_id(url: String) -> Result<u32> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: u32;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["selected_deck"].to_string().parse::<u32>().unwrap();
    } else {
        id = 0;
    }
    Ok(id)
}*/

pub async fn put_user(url: String, user: MemnixUser) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .put(url)
        .json(&user)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}