use crate::api::user::{fetch_user_id};
use crate::memnix::utils::ask;
use crate::memnix::utils::{access_forbidden_embed, beta_embed};
use crate::memnix::verifications::has_access;
use crate::utils::constants::{TEST_DECK};

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::api::mem::fetch_mem;

#[command]
async fn next(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user_id(
        format!(
            "http://127.0.0.1:1813/api/v1/users/discord/{:?}",
            msg.author.id.0
        )
        .to_string(),
    )
    .await
    .unwrap();

    if user_id == 0 {
        let _ = beta_embed(ctx, msg).await;
        return Ok(());
    }

    let access = has_access(user_id, TEST_DECK).await;
    if !access {
        let _ = access_forbidden_embed(ctx, msg).await;
        return Ok(());
    };

    let mem = fetch_mem(
        format!(
            "http://127.0.0.1:1813/api/v1/mems/user/{:?}/deck/{:?}/next", 
            user_id, TEST_DECK
        )
        .to_string(),
    )
    .await
    .unwrap(); 

    //TODO: Handle error

    let _ = ask(ctx, msg, &mem, user_id).await;

    Ok(())
}
