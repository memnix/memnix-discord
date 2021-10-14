
use crate::memnix::utils::access_forbidden_embed;
use crate::memnix::verifications::has_access;
use crate::api::user::fetch_user;
use crate::memnix::utils::ask;
use crate::utils::constants::TEST_DECK;


use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


use crate::api::mem::{fetch_mem};

#[command]
async fn next(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/user/discord/{:?}",
            msg.author.id.0
        )
        .to_string(),
    )
    .await
    .unwrap();

    let access = has_access(user_id, TEST_DECK).await;
    if !access {
        let _ = access_forbidden_embed(ctx, msg).await;
        return Ok(());
    }; 
    

    let mem = fetch_mem(
        format!(
            "http://127.0.0.1:1813/api/debug/user/{:?}/deck/1/next",
            user_id
        )
        .to_string(),
    )
    .await
    .unwrap();

    //TODO: Handle error

    let _ = ask(ctx, msg, &mem.card, user_id).await;

    Ok(())
}


