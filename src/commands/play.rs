use crate::core::game::connect_four::container::ConnectFourContainer;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::model::channel::ReactionType;

#[command]
fn c4(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut c4 = msg
        .channel_id
        .say(&ctx.http, "Loading <a:loading:617628744512700447>")
        .unwrap();

    add_react(ctx, &c4);

    c4.edit(&ctx, |m| {
        m.content("").embed(|e| {
            e.author(|a| {
                a.name("Connect Four")
                    .icon_url(&ctx.cache.read().user.face())
            })
            .color((255, 255, 0))
            .field("Awaiting for player", "Turns #", false)
            .image("https://i.imgur.com/Pnfyxmh.png")
        })
    })
    .unwrap();

    Ok(())
}

#[command]
fn games(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let c4 = data.get::<ConnectFourContainer>().unwrap().read();

    let _ = msg.reply(&ctx, format!("You have {} games ongoing!", c4.games.len()));

    Ok(())
}

fn add_react(ctx: &mut Context, msg: &Message) {
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304998428672010),
            name: Some("1_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304999938359306),
            name: Some("2_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304999883833347),
            name: Some("3_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304999057817601),
            name: Some("4_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304999171063809),
            name: Some("5_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304999451951105),
            name: Some("6_".to_string()),
        },
    )
    .unwrap();
    msg.react(
        &ctx,
        ReactionType::Custom {
            animated: false,
            id: EmojiId(621304998919274506),
            name: Some("7_".to_string()),
        },
    )
    .unwrap();
}
