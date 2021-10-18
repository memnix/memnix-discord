
use crate::api::user::fetch_user;
use crate::memnix::utils::{access_forbidden_embed, ask, beta_embed};
use crate::memnix::verifications::has_access;
use crate::utils::constants::{TEST_DECK, URL};

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


use crate::api::mem::{fetch_mem};

#[command]
async fn card(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(
        format!(
            "{:?}/v1/users/discord/{:?}", URL,
            msg.author.id.0
        )
        .to_string(),
    )
    .await
    .unwrap();

    if user_id == 0 {
        let _ = beta_embed(ctx, msg).await;
        return Ok(())
    }

    let access = has_access(user_id, TEST_DECK).await;
    if !access {
        let _ = access_forbidden_embed(ctx, msg).await;
        return Ok(());
    }; 

    let mem = fetch_mem(
        format!(
            "{:?}/v1/mems/user/{:?}/deck/{:?}/today", URL, user_id, TEST_DECK
        )
        .to_string(),
    )
    .await
    .unwrap();

    if mem.card.id == 0 {
        msg.reply(ctx, 
            "You dont have more cards to play for today ! But if you want to keep playing, you can use ~next !")
            .await?;
        return Ok(())
    }

    let _ = ask(ctx, msg, &mem, user_id).await;
    
    Ok(())
}
