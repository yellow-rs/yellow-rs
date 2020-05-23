use crate::core::messagecache_container::MessageCacheContainer;
use log::info;
use serenity::{
    async_trait,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::{ChannelId, MessageId},
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
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
    ) {
        let m_cache = ctx
            .cache
            .message(&channel_id, deleted_message_id)
            .await
            .unwrap();

        let mut m_container = ctx
            .data
            .read()
            .await
            .get::<MessageCacheContainer>()
            .unwrap()
            .write()
            .await
            .clone();

        m_container.insert(channel_id, m_cache);
    }
}
