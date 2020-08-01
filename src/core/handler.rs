use crate::core::game::c4::*;
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
    prelude::*,
};

pub struct ClientHandler;

impl ClientHandler {
    async fn message_add_internal(&self, ctx: Context, new_message: Message) -> Option<()> {
        if new_message.channel_id == ChannelId(617407223395647520) {
            let num = MessageId(new_message.content.parse::<u64>().unwrap());

            let data = ctx.data.read().await;
            let container_op = data.get::<C4ManagerContainer>()?;
            let mut write = container_op.write().await;
            if let Some(gem) = write.get_mut(&num) {
                gem.update_game(new_message.attachments[0].url.clone())
                    .await;
            }
        }
        None
    }
    async fn reaction_add_internal(&self, ctx: Context, add_reaction: Reaction) -> Option<()> {
        let msg = &ctx
            .cache
            .message(add_reaction.channel_id, add_reaction.message_id)
            .await?;

        if msg.is_own(&ctx.cache).await && add_reaction.user_id != msg.author.id {
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
                        let data = ctx.data.read().await;
                        let container_op = data.get::<C4ManagerContainer>()?;
                        unsafe {
                            container_op
                                .write()
                                .await
                                .reacted(msg.id, value, add_reaction.user_id)
                                .await;
                            let _ = add_reaction.delete(&ctx.http).await;
                        }
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
