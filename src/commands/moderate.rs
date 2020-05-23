use serenity::framework::{
    macros::command,
    standard::{Args, CommandResult},
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Flags a user, making them unable to embed links and send any media.")]
async fn flag(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[description("")]
async fn silent(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

#[command]
#[description("Kicks out a guild member.")]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}
