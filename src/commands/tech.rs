use crate::core::shardmanager_container::ShardManagerContainer;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
fn shards(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Application Status Report")
                .thumbnail(&ctx.cache.read().user.face())
                .colour((255, 255, 0))
                .field("Shard(s) in use", &ctx.cache.read().shard_count, false)
                .footer(|f| f.text("Powered by Allure™️  | ©️ 2020"))
        })
    });

    Ok(())
}

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    if let Some(manager) = ctx.data.read().get::<ShardManagerContainer>() {
        let _ = msg.reply(&ctx, "Shutting down!");
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager.");
    }

    Ok(())
}
