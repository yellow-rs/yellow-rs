use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

#[command]
fn verified(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(&ctx.cache.read().user.verified.unwrap())
    });

    Ok(())
}

#[command]
#[only_in(guilds)]
fn avatar(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    if let Some(ava) = msg.author.avatar_url() {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|e| e.name(msg.author.tag() + "\'s avatar!").icon_url(&ava))
                    .image(&ava)
                    .colour(Colour::ROSEWATER)
                    .footer(|f| {
                        f.text("Powered by Allure™️  | ©️ 2020")
                            .icon_url(ctx.cache.read().user.face())
                    })
            })
        });
    }

    Ok(())
}
