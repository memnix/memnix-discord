use core::time::Duration;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Color,
};

use crate::{
    api::revision::post_revision,
    models::{card::MemnixCard, revision::MemnixRevision},
};

pub async fn beta_embed(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(
        ctx,
        |m| {
            m.embed(|e| {
                e.color(Color::ROSEWATER);
                e.title("Access !");
                e.description("You don't have permission to use the bot as it's on beta mode only !\n
                If you think it's an error, contact Yume !\n\n
                If you want to become a beta tester, please contact Yume. If you don't know who the fuck is Yume, then that should be your starting point because you must know who is your god.");
                e
            })
        }).await?;
    Ok(())
}

pub async fn access_forbidden_embed(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(
        ctx,
        |m| {
            m.embed(|e| {
                e.color(Color::ROSEWATER);
                e.title("Access !");
                e.description("You don't have permission to play this deck !\n
                If you want to play this deck, use the following command: `~subscribe`
                If you think it's an error, contact Yume !\n\n
                `Deck permission hasn't been fully implemented in this beta yet. If you encounter any problem, contact Yume ASAP`");
                e
            })
        }).await?;
    Ok(())
}

async fn correct_embed(ctx: &Context ,msg: &Message, card: &MemnixCard, answer: String) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::DARK_GREEN);
                e.title("Correct");
                e.description(format!(
                    "Your answer: {}\nExpected Answer: **{}**",
                    answer,
                    card.answer.replace("\"", "")
                ));
                e
            })
        })
        .await?;
    Ok(())
}

async fn incorrect_embed(ctx: &Context, msg: &Message, card: &MemnixCard, answer: String) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::DARK_RED);
                e.title("Incorrect");
                e.description(format!(
                    "Your answer: {}\nExpected Answer: **{}**",
                    answer,
                    card.answer.replace("\"", "")
                ));
                e
            })
        })
        .await?;
    Ok(())
}

async fn question_embed(ctx: &Context, msg: &Message, card: &MemnixCard) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::BLURPLE);
                if !&card.image_url.contains("none") {
                    e.image(&card.image_url.replace("\"", ""));
                }

                e.title(format!("Card #{:?}", card.id));
                e.description(format!(
                    "** {}**\n\nIf you don't know the answer, type : `idk`",
                    card.question.replace("\"", "")
                ));
                if !&card.card_type.contains("none") {
                    e.footer(|f| f.text(&card.card_type.replace("\"", "")));
                };
                e
            })
        })
        .await?;
    Ok(())
}

pub async fn ask(ctx: &Context, msg: &Message, card: &MemnixCard, user_id: u32) -> CommandResult {
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
