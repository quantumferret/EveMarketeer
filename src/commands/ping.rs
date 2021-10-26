use serenity::framework::standard::{CommandResult, macros::command};
use serenity::client::{Context};
use serenity::model::channel::Message;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong!").await?;
    Ok(())
}