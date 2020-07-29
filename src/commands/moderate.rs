use serenity::framework::{
    macros::command,
    standard::{Args, CommandResult},
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Kicks out a guild member.")]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}
