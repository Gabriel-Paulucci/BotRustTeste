use serenity::{prelude::Context, framework::standard::{CommandResult, macros::{command, group}}, model::channel::{Message}};

#[group]
#[commands(ping)]
pub struct Util;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(ctx, |x| x
        .embed(|f| f
            .description(format!("**Pong!**")))).await?;

    Ok(())
}

