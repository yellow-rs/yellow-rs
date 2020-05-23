use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::core::messagecache_container::MessageCacheContainer;

#[command]
#[only_in(guilds)]
#[description("Sends an embed of user's avatar.")]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.image(msg.author.face())))
        .await;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Fetches the last edited/deleted message of a channel")]
async fn snipe(ctx: &Context, msg: &Message) -> CommandResult {
    let m_container = ctx
        .data
        .read()
        .await
        .get::<MessageCacheContainer>()
        .unwrap()
        .write()
        .await
        .clone();

    let recovered_m = m_container.get(&msg.channel_id).unwrap();
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!(
            "```{}: {}```",
            recovered_m.author, recovered_m.content
        ))
    });
    Ok(())
}
