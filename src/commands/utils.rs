use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::Context;

#[command]
#[min_args(1)]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut ns = fasteval::EmptyNamespace;

    let val = fasteval::ez_eval(args.rest(), &mut ns)?;

    //let _ = msg.reply(ctx,format!("\n```{}\n=\n{}```", args.rest(), val.to_string())).await;
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| m.embed(|e| e.field(args.rest(), val, true)))
        .await;

    Ok(())
}
