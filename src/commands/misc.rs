use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guilds)]
#[description("Sends an enmbed of user's avatar.")]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.broadcast_typing(&ctx.http).await;
    let ava = msg.author.face();
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.image(ava)))
        .await;

    Ok(())
}
