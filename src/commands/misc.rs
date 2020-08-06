use crate::core::utils::parse_member::*;
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
#[min_args(2)]
#[only_in(guilds)]
#[description("Sends a message on behalf of a user.")]
async fn sudo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.delete(&ctx.http).await?;

    let mem = args
        .single_quoted::<String>()
        .unwrap_or_else(|_| msg.author.mention());

    let backup = &ctx
        .http
        .get_member(msg.guild_id.unwrap().0, msg.author.id.0)
        .await
        .unwrap();
    let member = parse_member(ctx, msg.clone(), mem)
        .await
        .unwrap_or_else(move |_| backup.clone());
    let name = " ͔".to_owned() + &member.display_name();
    let webhook = &ctx
        .http
        .create_webhook(msg.channel_id.0, &json!({ "name": name }))
        .await?;

    webhook
        .execute(&ctx.http, false, |w| {
            w.avatar_url(member.user.face())
                .content(args.clone().remains().unwrap())
        })
        .await?;

    webhook.delete(&ctx.http).await?;

    let _ = ChannelId(617407223395647520)
            .send_message(&ctx.http, move |m| {
                m.content(msg.id.0).embed(|e| {
                    e.title("New sudo command used!")
                        .field("Command author", msg.author.mention(), true)
                        .field("Sent as user", member.mention(), true)
                        .field("Contents", format!("```\n{}\n```", args.remains().unwrap()), false)
                })
            })
            .await;

    
    Ok(())
}

#[help]
#[individual_command_tip = "Hello!
If youd like to get more information about a specific command or group, you can just pass it as a command argument.
All the command examples through out the help will be shown without prefix, use whatever prefix is configured on the server instead.
"]
// This is the text that gets displayed when a given parameter was not found for information.
#[command_not_found_text = "Could not find: `{}`."]
// This is the ~~strikethrough~~ text.
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
// This is the level of similarities between the given argument and possible other arguments.
// This is used to give suggestions in case of a typo.
#[max_levenshtein_distance(3)]
// This makes it so specific sections don't get showed to the user if they don't have the
// permission to use them.
#[lacking_permissions = "Hide"]
// In the case of just lacking a role to use whatever is necessary, nothing will happen when
// setting it to "Nothing", rn it just strikes the option.
#[lacking_role = "Hide"]
// In the case of being on the wrong channel type (either DM for Guild only commands or vicecersa)
// the command will be ~~striked~~
#[wrong_channel = "Strike"]
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
