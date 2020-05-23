use crate::core::shardmanager_container::ShardManagerContainer;

use serenity::{
    client::bridge::gateway::ShardId,
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
            ctx,
            format!(
                "There are currently {} shard(s) in use",
                ctx.cache.shard_count().await
            ),
        )
        .await;

    Ok(())
}

#[command]
#[aliases("stat")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = chrono::Utc::now();
    let mut msg = msg.channel_id.say(&ctx, "pong!").await.unwrap();
    let finish = chrono::Utc::now();
    let lping = ((finish.timestamp() - now.timestamp()) * 1000)
        + (i64::from(finish.timestamp_subsec_millis()) - i64::from(now.timestamp_subsec_millis()));
    let shard_manager = ctx
        .data
        .read()
        .await
        .get::<ShardManagerContainer>()
        //.unwrap()
        .ok_or_else(|| "Failed to get ClientShardManager.")?
        .clone();
    let shard_latency = shard_manager
        .lock()
        .await
        .runners
        .lock()
        .await
        .get(&ShardId(ctx.shard_id))
        //.unwrap()
        .ok_or_else(|| "Failed to get Shard.")?
        .latency
        //.unwrap()
        .ok_or_else(|| "Failed to get latency from shard.")?
        .as_millis();

    let _ = msg
        .edit(ctx, |m| {
            m.content("").embed(|e| {
                e.field("Rest API", lping, true)
                    .field("Shard Latency", shard_latency, true)
            })
        })
        .await;

    Ok(())
}
