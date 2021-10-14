use std::time::Duration;

use crate::api::access::post_access;
use crate::api::deck::fetch_deck;
use crate::api::user::fetch_user;
use crate::models::access::MemnixAccess;
use crate::models::mem::MemnixMem;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn subscribe(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/user/discord/{:?}",
            msg.author.id.0
        )
        .to_string()
    )
    .await
    .unwrap();

    msg.reply(ctx, "What deck would you like to subscribe to ? (Type the deck id ! Ex: 1)").await?;


    let answer = match msg
        .channel_id
        .await_reply(&ctx)
        .timeout(Duration::from_secs(60))
        .author_id(msg.author.id)
        .await
    {
        Some(answer) => answer.content.clone(),
        None => {
            msg.channel_id
                .say(&ctx.http, "No answer within 60 seconds")
                .await?;
            return Ok(());
        }
    };

    if !answer.parse::<u32>().is_ok() {
        msg.channel_id
                .say(&ctx.http, "Please provide a deck ID ! That should be an integer.")
                .await?;
            return Ok(());
    };

    let deck = fetch_deck(format!("http://127.0.0.1:1813/api/v1/deck/id/{:?}", answer.parse::<u32>().unwrap())).await.unwrap();
    if deck.id == 0 || deck.private {
        println!("http://127.0.0.1:1813/api/v1/deck/id/{:?}", answer.parse::<u32>());
        msg.channel_id
        .say(&ctx.http, format!("This deck ID {:?} hasn't been found or you don't have access to this deck (it might be private)", answer))
        .await?;
        return Ok(());
    };

    let access = MemnixAccess {
        user_id: user_id, 
        deck_id: deck.id, 
        permission: 1 
    };

    let _ = post_access("http://127.0.0.1:1813/api/v1/access/new".to_string(), access).await;
    

    msg.channel_id
    .say(&ctx.http, "You are now sub to this deck ! Enjoy playing !")
    .await?;

    Ok(())
}
