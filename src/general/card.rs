use crate::api::revision::post_revision;
use crate::api::user::fetch_user;
use crate::models::revision::{MemnixRevision};
use std::time::Duration;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::api::card::fetch_card;

#[command]
async fn card(ctx: &Context, msg: &Message) -> CommandResult {
    let user_id = fetch_user(format!("http://127.0.0.1:1813/api/v1/user/discordid/{:?}", msg.author.id.0).to_string()).await.unwrap();

    let response = fetch_card(format!("http://127.0.0.1:1813/api/debug/user/{:?}/deck/1/next", user_id).to_string()).await;
    //TODO: Handle error
    let card = response.unwrap();
    let question = card.question;

    msg.reply(ctx, format!("**Card #{:?}**\n\n> Question: *{:?}*", card.id, question.replace("\"", ""))).await?;
    
    let answer = match msg.channel_id.await_reply(&ctx).timeout(Duration::from_secs(60)).author_id(msg.author.id).await {
        Some (answer) =>  answer.content.clone(),
        None => {
            msg.channel_id
            .say(&ctx.http, "No answer within 60 seconds")
            .await?;
            return Ok(());
        }
    };
    let result: bool;
    let result_int: i8;
    if (answer.parse::<f64>().is_ok() && answer.eq(&card.answer.replace("\"", "")))
    || (!answer.parse::<f64>().is_ok() && answer.to_lowercase().contains(&card.answer.replace("\"", "").to_lowercase())) {
        result = true;
        result_int = 1;
        msg.channel_id.say(&ctx.http, format!("**Correct !**\n\nYour Answer: {}\nExpected Answer: {}", answer, card.answer.replace("\"", ""))).await?;
    } else {
       result = false;
       result_int = 0;

       msg.channel_id.say(&ctx.http, format!("**Incorrect !**\n\nYour Answer: {}\nExpected Answer: {}", answer, card.answer.replace("\"", ""))).await?;
    }
    let revision = MemnixRevision {
        user_id: user_id,
        card_id: card.id,
        result: result,
        result_int: result_int
    };

    let _ = post_revision("http://127.0.0.1:1813/api/v1/revision/new".to_string(), revision).await;
    
    Ok(())
}
