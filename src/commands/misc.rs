use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde_json::json;

#[command]
#[only_in(guilds)]
#[description("Sends a message on behalf of a user.")]
async fn sudo(ctx: &Context, msg:&Message) -> CommandResult {
    let name = " ͔".to_owned() + &msg.author.name;
    let webhook = &ctx.http.create_webhook(msg.channel_id.0, &json!({"name": name})).await?;

    webhook.execute(&ctx.http, false, |w| {
        w.avatar_url(msg.author.face()).content("Here's a webhook")
    })
    .await?;

    webhook.delete(&ctx.http).await?;
    Ok(())
}

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

