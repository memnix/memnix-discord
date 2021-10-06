use crate::models::user::MemnixUser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub async fn fetch_user(url: String) -> Result<i8> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: i8;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<i8>().unwrap();
    } else {
        id = 0;
    }

    Ok(id)
}