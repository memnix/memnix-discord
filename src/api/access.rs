use crate::models::{access::MemnixAccess};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_access(url: String) ->  Result<MemnixAccess> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut memnixaccess = MemnixAccess {
        user_id: 0,
        deck_id: 0,
        permission: 0,
    };

    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        memnixaccess = MemnixAccess {
            user_id: echo_json["data"]["user_id"].to_string().parse::<u32>().unwrap(),
            deck_id: echo_json["data"]["deck_id"].to_string().parse::<u32>().unwrap(),
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
