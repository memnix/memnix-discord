
use crate::models::revision::MemnixRevision;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn post_revision(url: String, revision: MemnixRevision) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&revision)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}