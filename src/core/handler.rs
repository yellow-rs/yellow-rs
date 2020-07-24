use crate::core::game::c4::*;
use log::info;
use serenity::{
    async_trait,
    client::Context,
    model::{
        channel::{Reaction, ReactionType},
        event::ResumedEvent,
        gateway::Ready,
    },
    prelude::*,
};

pub struct ClientHandler;

#[async_trait]
impl EventHandler for ClientHandler {
    async fn ready(&self, ctx: Context, _: Ready) {
        info!("{} has logged in!", ctx.cache.current_user().await.tag());
    }
    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        info!("{} has resumed!", ctx.cache.current_user().await.tag());
    }
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let msg_req = &ctx
            .cache
            .message(add_reaction.channel_id, add_reaction.message_id)
            .await;

        if let Some(msg) = msg_req {
            if msg.is_own(&ctx.cache).await && add_reaction.user_id != msg.author.id {
                if let ReactionType::Custom {
                    animated: _,
                    id: _,
                    name: e_name,
                } = add_reaction.emoji.clone()
                {
                    if let Some(name) = e_name {
                        let first_char = name.chars().next().unwrap();
                        if first_char.is_digit(10) {
                            let value = first_char.to_digit(10).unwrap();

                            let data = ctx.data.read().await;
                            let container = data.get::<C4ManagerContainer>().unwrap();
                            let manager = container.read().await;

                            manager.reacted(msg.id, value as usize, add_reaction.user_id);
                            let _ = add_reaction.delete(&ctx.http).await;
                        }
                    }
                }
            }
        }
    }
}
