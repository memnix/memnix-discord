type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_access(url: String) ->  Result<MemnixAccess> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut memnixcard = MemnixCard {
        user_id: 0,
        deck_id: 0,
        permission: 0,
        id: 0,
    };

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixaccess = MemnixAccess {
            id: echo_json["data"]["id"].to_string().parse::<i8>().unwrap(),
            user_id: echo_json["data"]["user_id"].to_string().parse::<i8>().unwrap(),
            deck_id: echo_json["data"]["deck_id"].to_string().parse::<i8>().unwrap(),
            permission: echo_json["data"]["permission"].to_string().parse::<i8>().unwrap(),  
        };
    };

    Ok(memnixaccess)
}


pub async fn post_access(url: String, access: MemnixAccess) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&access)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
