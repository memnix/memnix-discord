use std::time::Duration;

use crate::api::{deck::{fetch_deck, post_deck}, user::fetch_user};


use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn subscribe(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/users/discord/{:?}", 
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

    let deck = fetch_deck(format!("http://127.0.0.1:1813/api/v1/decks/id/{:?}", answer.parse::<u32>().unwrap())).await.unwrap();
    if deck.id == 0 || deck.status < 2 {
        msg.channel_id
        .say(&ctx.http, format!("This deck ID {:?} hasn't been found or you don't have access to this deck (it might be private)", answer))
        .await?;
        return Ok(());
    };

    let _ = post_deck(format!("http://127.0.0.1:1813/api/v1/decks/{:?}/user/{:?}/subscribe", deck.id, user_id), deck).await;
    

    msg.channel_id
    .say(&ctx.http, "You are now sub to this deck ! Enjoy playing !")
    .await?;

    Ok(())
}
