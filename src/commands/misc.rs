use serde_json::json;
use serenity::framework::standard::{
    help_commands,
    macros::{command, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::collections::HashSet;

#[command]
#[only_in(guilds)]
#[description("Sends a message on behalf of a user.")]
async fn sudo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.delete(&ctx.http).await?;
    let name = " ͔".to_owned() + &msg.author.name;
    let webhook = &ctx
        .http
        .create_webhook(msg.channel_id.0, &json!({ "name": name }))
        .await?;

    webhook
        .execute(&ctx.http, false, |w| {
            w.avatar_url(msg.author.face()).content(args.rest())
        })
        .await?;

    webhook.delete(&ctx.http).await?;
    Ok(())
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Sends an embed of user's avatar.")]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.image(msg.author.face())))
        .await?;

    Ok(())
}
