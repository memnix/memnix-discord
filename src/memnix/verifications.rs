use crate::{api::access::fetch_access, utils::constants::URL};

pub async fn has_access(user_id: u32, deck_id: u32) -> bool {
    let access = fetch_access(
        format!(
            "{:?}/v1/accesses/user/{:?}/deck/{:?}", URL,
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
