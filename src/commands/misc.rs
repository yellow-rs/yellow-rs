use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

#[command]
fn avatar(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http).ok();
    if let Some(ava) = &msg.author.avatar_url() {
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(msg.author.tag() + "\'s avatar!")
                    .url(&ava)
                    .image(&ava)
                    .colour((255, 255, 0))
                    .footer(|f| f.text("Powered by Allure™️  | ©️ 2020"))
            })
        });
    }

    Ok(())
}
