use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Ping").await?;
    Ok(())
}
