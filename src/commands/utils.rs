use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::Context;

use crate::core::eval;

#[command]
#[min_args(1)]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = match eval::exec(&args.rest()[..]) {
        Ok(val) => val,
        Err(why) => why.to_string()
    };

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.field(args.rest(), result, true)))
        .await;

    Ok(())
}
