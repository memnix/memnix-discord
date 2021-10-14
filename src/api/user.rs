type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_user(url: String) -> Result<u32> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: u32;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<u32>().unwrap();
    } else {
        id = 0;
    }

    Ok(id)
}
