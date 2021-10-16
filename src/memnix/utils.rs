use rand::{Rng, seq::SliceRandom};
use rand::thread_rng;
use core::time::Duration;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Color,
};

use crate::models::mem::MemnixMem;
use crate::{api::{answer::fetch_answers, revision::post_revision}, models::{answer::MemnixAnswer, card::MemnixCard, revision::MemnixRevision}};

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

async fn correct_embed(ctx: &Context ,msg: &Message, answer: String, correct_answer: String) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::DARK_GREEN);
                e.title("Correct");
                e.description(format!(
                    "Your answer: {}\nExpected Answer: **{}**",
                    answer,
                    correct_answer.replace("\"", "")
                ));
                e
            })
        })
        .await?;
    Ok(())
}

async fn incorrect_embed(ctx: &Context, msg: &Message, answer: String, correct_answer: String) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.color(Color::DARK_RED);
                e.title("Incorrect");
                e.description(format!(
                    "Your answer: {}\nExpected Answer: **{}**",
                    answer,
                    correct_answer.replace("\"", "")
                ));
                e
            })
        })
        .await?;
    Ok(())
}

async fn level3(ctx: &Context, msg: &Message, card: &MemnixCard, user_id: u32) -> CommandResult {
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

        let _ = wait_answer(ctx, msg, card, card.answer.to_string(), user_id, 3).await;
    Ok(())
}

pub async fn wait_answer(ctx: &Context, msg: &Message, card: &MemnixCard, correct_answer: String, user_id: u32, level: u8) -> CommandResult {
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
    let quality: i8;
    if (answer.parse::<f64>().is_ok() && answer.eq(&correct_answer.replace("\"", "")))
        || (!answer.parse::<f64>().is_ok()
            && answer
                .to_lowercase()
                .contains(&correct_answer.replace("\"", "").to_lowercase()))
    {
        result = true;
        result_int = 1;
        if level <=3 {
            quality = 4;
        } else {
            quality = 5;
        };
        let _ = correct_embed(ctx, msg, answer, correct_answer).await;
    } else {
        result = false;
        result_int = 0;

        if answer.contains("idk")  {
            quality = 0;
        } else if level == 1 {
            quality = 1;
        }else {
            quality = 3;
        }

        let _ = incorrect_embed(ctx, msg, answer, correct_answer).await;
    }
    let revision = MemnixRevision {
        user_id: user_id,
        card_id: card.id,
        result: result,
        result_int: result_int,
        quality: quality,
    };

    let _ = post_revision(
        "http://127.0.0.1:1813/api/v1/revision/new".to_string(),
        revision,
    )
    .await;
    
    Ok(())
}

pub async fn ask_level1(ctx: &Context, msg: &Message, card: &MemnixCard, mut answers: Vec<MemnixAnswer>, user_id: u32) -> CommandResult {
    let mut to_display: Vec<&MemnixAnswer> = Vec::new();

    let answer = MemnixAnswer {
        card_id: card.id,
        answer: card.answer.to_string(),
    };

    answers.shuffle(&mut thread_rng());
    for x in 0..3 {
        to_display.push(answers.get(x).unwrap());
    };
    let index = thread_rng().gen_range(0..=3);
    to_display.insert( index ,&answer);

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
                e.field("1", to_display.get(0).unwrap().answer.replace("\"", ""), false);
                e.field("2", to_display.get(1).unwrap().answer.replace("\"", ""), false);
                e.field("3", to_display.get(2).unwrap().answer.replace("\"", ""), false);
                e.field("4", to_display.get(3).unwrap().answer.replace("\"", ""), false);

                if !&card.card_type.contains("none") {
                    e.footer(|f| f.text("Type the right number"));
                };
                e
            })
        })
        .await?;

        let _ = wait_answer(ctx, msg, card, (index+1).to_string(), user_id, 1).await;
           Ok(())
}

pub async fn ask(ctx: &Context, msg: &Message, mem: &MemnixMem, user_id: u32) -> CommandResult {
    
    if mem.total < 3 || mem.efactor <= 1.4 || mem.quality <= 1 || mem.repetition < 2 {
        let answers = fetch_answers(format!("http://127.0.0.1:1813/api/v1/answer/card/{:?}", mem.card.id)).await.unwrap();
        if answers.len() >= 3 {
            let _ = ask_level1(ctx, msg, &mem.card, answers, user_id).await;
        } else {
            let _ = level3(ctx, msg, &mem.card, user_id).await;
        };
    } else {
        let _ = level3(ctx, msg, &mem.card, user_id).await;

    };
    Ok(())
}
