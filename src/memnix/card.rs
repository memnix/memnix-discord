
use crate::api::user::fetch_user;
use crate::memnix::utils::ask;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


use crate::api::card::fetch_card;

#[command]
async fn card(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/user/discord/{:?}",
            msg.author.id.0
        )
        .to_string(),
    )
    .await
    .unwrap();

    let card = fetch_card(
        format!(
            "http://127.0.0.1:1813/api/debug/user/{:?}/deck/1/today",
            user_id
        )
        .to_string(),
    )
    .await
    .unwrap();

    if card.id == 0 {
        msg.reply(ctx, 
            "You dont have more cards to play for today ! But if you want to keep playing, you can use ~next !")
            .await?;
        return Ok(())
    }

    let _ = ask(ctx, msg, &card, user_id).await;
    
    Ok(())
}
