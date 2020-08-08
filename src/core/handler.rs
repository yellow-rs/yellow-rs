use crate::core::{game::c4::C4ManagerContainer, db::DatabaseWrapper};
use log::info;
use serenity::{
    async_trait,
    client::Context,
    model::{
        channel::{Message, Reaction, ReactionType},
        event::ResumedEvent,
        gateway::Ready,
        id::{ChannelId, MessageId},
    },
    utils::Color,
    prelude::*,
};

pub struct ClientHandler;

impl ClientHandler {
    async fn message_add_internal(&self, ctx: Context, new_message: Message) -> Option<()> {
        if new_message.channel_id == ChannelId(617407223395647520) {
            let data = ctx.data.read().await;
            let container_op = data.get::<C4ManagerContainer>()?;
            let read = container_op.read().await;

            let msg_id = MessageId(new_message.content.parse::<u64>().unwrap());


            // XXX: clean up this code; the use of locks makes it messy
            // for the motivation of keeping critical sections short

            // !! Start of critical section
            let mut gem = read.get(&msg_id)?.write().await;
            if let Some((player_a, player_b, result)) = gem
                    .update_game(&new_message.attachments[0].url)
                    .await {
                        let player_a_mention = player_a.name.clone();
                        let player_b_mention = player_b.name.clone();
                        let player_a_id = player_a.id;
                        let player_b_id = player_b.id;

                        // !! End of critical section
                        std::mem::drop(gem);

                        // Game has ended here, modify db
                        let db = data.get::<DatabaseWrapper>()?;
                        let (a_change, a_final, b_change, b_final) = db.update_score(player_a_id.0 as i64, player_b_id.0 as i64, result).await;

                        fn change_word(amount: i32) -> &'static str {
                            if amount < 0 {
                                "**DECREASED**"
                            } else {
                                "**INCREASED**"
                            }
                        }

                        let results_desc = format!(
                            "Results from {} and {}'s game",
                            player_a_id.mention(),
                            player_b_id.mention()
                        );

                        let player_a_score = format!(
                            "Score {} by `{}`, now at a total of `{}`.",
                            change_word(a_change),
                            a_change.abs(),
                            a_final
                        );

                        let player_b_score = format!(
                            "Score {} by `{}`, now at a total of `{}`.",
                            change_word(b_change),
                            b_change.abs(),
                            b_final
                        );

                        // !! Start of critical section
                        read.get(&msg_id)?.write().await.send_embed(|e|
                            e.title("Results")
                            .description(results_desc)
                            .field(
                                player_a_mention,
                                player_a_score,
                                false
                            )
                            .field(
                                player_b_mention,
                                player_b_score,
                                false
                            )
                            .color(Color::from_rgb(43, 82, 224))
                        ).await;
                        // !! End of critical section
                    };
        }
        None
    }
    async fn reaction_add_internal(&self, ctx: Context, add_reaction: Reaction) -> Option<()> {
        let data = ctx.data.read().await;
        let container_op = data.get::<C4ManagerContainer>()?;

        if container_op
            .read()
                .await
                .contains_key(&add_reaction.message_id)
        {
            let msg = add_reaction.message(&ctx.http).await.unwrap();
            if let ReactionType::Custom {
                animated: _,
                id: _,
                name,
            } = add_reaction.emoji.clone()
            {
                let first_char = name?.chars().next().unwrap();
                if first_char.is_digit(10) {
                    let value = first_char.to_digit(10).unwrap() as usize;
                    if value > 0 && value < 8 {
                        let cn = container_op.read().await;
                        let gem = cn.get(&msg.id)?;
                        unsafe {
                            gem.write()
                                .await
                                .move_coin(value, add_reaction.user_id.unwrap())
                                .await;
                            }
                        let _ = add_reaction.delete(&ctx.http).await;
                    }
                }
            }
        }
        None
    }
}

#[async_trait]
impl EventHandler for ClientHandler {
    async fn ready(&self, ctx: Context, _: Ready) {
        info!("{} has logged in!", ctx.cache.current_user().await.tag());
    }
    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        info!("{} has resumed!", ctx.cache.current_user().await.tag());
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let _ = self.reaction_add_internal(ctx, add_reaction).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let _ = self.message_add_internal(ctx, new_message).await;
    }
}
