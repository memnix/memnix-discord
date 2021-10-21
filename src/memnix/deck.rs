use std::time::Duration;

use crate::api::{access::fetch_access, deck::{fetch_deck, fetch_decks, post_deck}, user::{fetch_user, fetch_user_id, put_user}};


use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn deck(ctx: &Context, msg: &Message) -> CommandResult {
    let mut user = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/users/discord/{:?}", 
            msg.author.id.0
        )
        .to_string()
    )
    .await
    .unwrap();
    
    // TODO: Handle errors

    msg.reply(ctx, "What deck would you like to select ? (Type the deck id ! Ex: 1)").await?;

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

    let access = fetch_access(
        format!("http://127.0.0.1:1813/api/v1/accesses/user/{:?}/deck/{:?}", user.id, answer.parse::<u32>().unwrap())).await.unwrap();

    if access.permission == 0 {
        msg.channel_id
        .say(&ctx.http, format!("This deck ID {:?} hasn't been found or you don't have access to this deck (it might be private)", answer))
        .await?;
    return Ok(());
    }

    let deck = fetch_deck(format!("http://127.0.0.1:1813/api/v1/decks/id/{:?}", answer.parse::<u32>().unwrap())).await.unwrap();
    if deck.id == 0 || deck.status == 0 {
        msg.channel_id
        .say(&ctx.http, format!("This deck ID {:?} hasn't been found or you don't have access to this deck (it might be private)", answer))
        .await?;
        return Ok(());
    };

    user.selected_deck = deck.id;

    let _ = put_user(
        format!("http://127.0.0.1:1813/api/v1/users/id/{:?}", user.id),
        user,
    )
    .await;

    msg.channel_id
    .say(&ctx.http, "You are now using this deck ! Enjoy playing !")
    .await?;

    Ok(())
}


#[command]
async fn mydecks(ctx: &Context, msg: &Message) -> CommandResult {
    let user = fetch_user(
        format!(
            "http://127.0.0.1:1813/api/v1/users/discord/{:?}", 
            msg.author.id.0
        )
        .to_string()
    )
    .await
    .unwrap();
    
    // TODO: Handle errors

    let decks = fetch_decks(format!("http://127.0.0.1:1813/api/v1/decks/user/{:?}", user.id)).await.unwrap();
    let mut final_str = "__Subscribed Decks__\n".to_owned();
    for val in decks.iter() {
        let temp: String;
        if val.id == user.selected_deck {
             temp = format!("Deck #**{:?}** | *{:?}* `selected`\n", val.id, val.deck_name.replace("\"", ""));
        } else {
             temp = format!("Deck #**{:?}** | *{:?}*\n", val.id, val.deck_name.replace("\"", ""));
        };
        final_str.push_str(&temp);
    };

    // final_str.push_str(&format!("\n__Selected Deck__: **{:?}**", user.selected_deck));

    msg.channel_id
    .say(&ctx.http, final_str)
    .await?;

    Ok(())
}

#[command]
async fn decks(ctx: &Context, msg: &Message) -> CommandResult {
    
    let decks = fetch_decks(format!("http://127.0.0.1:1813/api/v1/decks/public/")).await.unwrap();
    let mut final_str = "__Public Decks__\n".to_owned();
    for val in decks.iter() {
        let temp = format!("Deck #**{:?}** | *{:?}*\n", val.id, val.deck_name.replace("\"", ""));
        final_str.push_str(&temp);
    };

    msg.channel_id
    .say(&ctx.http, final_str)
    .await?;

    Ok(())
}

#[command]
async fn subscribe(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user_id(
        format!(
            "http://127.0.0.1:1813/api/v1/users/discord/{:?}", 
            msg.author.id.0
        )
        .to_string()
    )
    .await
    .unwrap();

        // TODO: Handle errors


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
