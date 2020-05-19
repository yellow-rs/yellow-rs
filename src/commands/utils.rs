use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::Context;

#[command]
#[aliases("+")]
#[min_args(2)]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut sum: f64 = 0.0;
    let mut expression: String = String::from("```");

    sum += args.single::<f64>().unwrap();
    expression += &sum.to_string();

    while !args.is_empty() {
        let num = args.single::<f64>().unwrap();

        sum += &num;
        expression += " + ";
        expression += &num.to_string();
    }

    expression += " = ";
    expression += &sum.to_string();
    expression += "```";

    let _ = msg.channel_id.say(&ctx.http, expression).await;

    Ok(())
}
