use log::info;
use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
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
}
