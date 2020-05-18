use log::info;
use serenity::{
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

pub struct ClientHandler;

impl EventHandler for ClientHandler {
    fn ready(&self, ctx: Context, _: Ready) {
        info!("{} has logged in!", ctx.cache.read().user.tag());
    }
    fn resume(&self, ctx: Context, _: ResumedEvent) {
        info!("{} has resumed!", ctx.cache.read().user.tag());
    }
}
