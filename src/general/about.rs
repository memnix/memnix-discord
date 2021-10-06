use crate::utils::constants::*;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let kitsu = &ctx.cache.current_user().await;
    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.thumbnail(kitsu.avatar_url().unwrap());
            e.description(format!(r#"**Memnix** beta testing discord bot"#));
            e.color(Color::BLURPLE);
            e.title("About");
            e.field("Version", format!("``` {}```", MEMNIXBOT_VERSION), true);
            e.field("API", format!("``` {} ```", API_VERSION), true);
            e
        })
    }).await?;
    Ok(())
}
