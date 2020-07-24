use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::ReactionType;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::core::game::c4::*;

#[command]
#[aliases("c4")]
#[description("Initializes a Connect 4 session.")]
async fn connect_four(ctx: &Context, msg: &Message) -> CommandResult {
    let gem = msg
        .channel_id
        .send_message(&ctx.http, |m| m.content("Initializing"))
        .await?;

    add_react(ctx, &gem).await;

    let data = ctx.data.read().await;
    let container = data.get::<C4ManagerContainer>().unwrap();
    let mut c4manager = container.write().await;

    c4manager.new_game(gem);

    Ok(())
}
/*
#[command]
#[owners_only]
#[aliases("g")]
#[description("Retrieves numbers of persisting games in cache.")]
async fn games(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let c4 = data.get::<ConnectFourContainer>().unwrap().read().await;

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Games persisting")
                    .field("Connect 4", c4.games.len(), true)
                    .field("Go 囲碁", "#", true)
            })
        })
        .await;

    Ok(())
}
*/
async fn add_react(ctx: &Context, msg: &Message) {
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304998428672010),
                name: Some("1_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304999938359306),
                name: Some("2_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304999883833347),
                name: Some("3_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304999057817601),
                name: Some("4_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304999171063809),
                name: Some("5_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304999451951105),
                name: Some("6_".to_string()),
            },
        )
        .await;
    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: EmojiId(621304998919274506),
                name: Some("7_".to_string()),
            },
        )
        .await;
}
