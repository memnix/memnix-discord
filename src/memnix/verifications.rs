use crate::api::access::fetch_access;

pub async fn has_access(user_id: u32, deck_id: u32) -> bool {
    let access = fetch_access(
        format!(
            "http://127.0.0.1:1813/api/v1/access/user/{:?}/deck/{:?}",
            user_id, deck_id
        )
        .to_string(),
    )
    .await
    .unwrap();
    if access.permission > 0 {
        return true;
    } else {
        return false;
    }
}
