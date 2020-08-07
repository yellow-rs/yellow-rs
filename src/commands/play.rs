use crate::core::{db::{DatabaseWrapper, ranking}, game::c4::*};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::ReactionType,
    model::prelude::*,
    prelude::*,
    utils::Color
};
use std::sync::Arc;

#[command]
#[aliases("lb")]
#[description("Show top 10 players.")]
async fn leaderboard(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db_wrapper: &DatabaseWrapper = data.get::<DatabaseWrapper>().unwrap();
    let top_10 = db_wrapper.get_top_n(10).await;
    let top_10_string: String =
        top_10.into_iter()
        .map(|row| {
            let id: i64 = row.get("id");
            let scores: Vec<ranking> = row.get("rankings");
            format!(
                "<@{}> - **`{}`** points\n",
                id,
                scores.last().unwrap().rank
            )
        }).collect();

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Top 10 players")
                .field("Players", top_10_string, false)
                .color(Color::from_rgb(33, 255, 92))
        })
    }).await;
    Ok(())
}

#[command]
#[aliases("c4")]
#[description("Initializes a Connect 4 session.")]
async fn connect_four(ctx: &Context, msg: &Message) -> CommandResult {
    let mut gem = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Initializing <a:loading:617628744512700447>")
        })
    .await?;

    add_react(ctx, &gem).await;

    let _ = gem
        .edit(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Connect Four™")
                    .field("New Player's turn!", "React to start!", false)
                    .image("https://imgur.com/R0THwNS.png")
                    .url("https://imgur.com/R0THwNS.png")
                    .footer(|f| {
                        f.text("| Don't Report bugs | Version 0.1.1 | React to place coin |")
                    })
            })
            .content("​")
        })
    .await;

    let data = ctx.data.read().await;
    let c4_container = data.get::<C4ManagerContainer>().unwrap();

    c4_container.write().await.insert(
        gem.id,
        Arc::new(RwLock::new(C4Instance::new(gem, Arc::clone(&ctx.http)))),
    );

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
