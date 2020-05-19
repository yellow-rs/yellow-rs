use crate::core::shardmanager_container::ShardManagerContainer;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(manager) = ctx.data.read().await.get::<ShardManagerContainer>() {
        let _ = msg.reply(ctx, "Shutting down!").await;
        manager.lock().await.shutdown_all().await;
    } else {
        let _ = msg
            .reply(ctx, "There was a problem getting the shard manager.")
            .await;
    }

    Ok(())
}

#[command]
#[description = "Reports number of shards in use."]
#[aliases("shard")]
async fn shards(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            format!(
                "There are currently {} shard(s) in use",
                ctx.cache.shard_count().await
            ),
        )
        .await;

    Ok(())
}
