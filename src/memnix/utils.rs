use core::time::Duration;
use serenity::{client::Context, framework::standard::CommandResult, model::channel::{Message}, utils::Color};

use crate::{api::revision::post_revision, models::{card::MemnixCard, revision::MemnixRevision}};

async fn correct_embed(ctx: &Context,msg: &Message, card: &MemnixCard, answer: String )  -> CommandResult {
    msg.channel_id.send_message(
        ctx,
        |m| {
            m.embed(|e| {
                e.color(Color::DARK_GREEN);
                e.title("Correct");
                e.description(format!("Your answer: {}\nExpected Answer: **{}**", answer, card.answer.replace("\"", "")
            ));
                e
            })
        }).await?;
        
    Ok(())
}

async fn incorrect_embed(ctx: &Context,msg: &Message, card: &MemnixCard, answer: String )  -> CommandResult {
    msg.channel_id.send_message(
        ctx,
        |m| {
            m.embed(|e| {
                e.color(Color::DARK_RED);
                e.title("Incorrect");
                e.description(format!("Your answer: {}\nExpected Answer: **{}**", answer, card.answer.replace("\"", "")
            ));
                e
            })
        }).await?;
        
    Ok(())
}


async fn question_embed(ctx: &Context, msg: &Message, card: &MemnixCard) -> CommandResult {

    msg.channel_id.send_message(
        ctx,
        |m| {
            m.embed(|e| {
                e.color(Color::BLURPLE);
                e.title(format!("Card #{:?}", card.id));
                e.description(format!("** {}**\n\nIf you don't know the answer, type : `idk`", card.question));
                e
            })
        }).await?;
        
    Ok(())
}


pub async fn ask(ctx: &Context, msg: &Message, card: &MemnixCard, user_id: i8)-> CommandResult {

    let _ = question_embed(ctx, msg, card).await;

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
    let result: bool;
    let result_int: i8;
    if (answer.parse::<f64>().is_ok() && answer.eq(&card.answer.replace("\"", "")))
        || (!answer.parse::<f64>().is_ok()
            && answer
                .to_lowercase()
                .contains(&card.answer.replace("\"", "").to_lowercase()))
    {
        result = true;
        result_int = 1;
        let _ = correct_embed(ctx, msg, card, answer).await;
    } else {
        result = false;
        result_int = 0;

        let _ = incorrect_embed(ctx, msg, card, answer).await;
    }
    let revision = MemnixRevision {
        user_id: user_id,
        card_id: card.id,
        result: result,
        result_int: result_int,
        quality: 0,
    };

    let _ = post_revision(
        "http://127.0.0.1:1813/api/v1/revision/new".to_string(),
        revision,
    )
    .await;

    Ok(())
}