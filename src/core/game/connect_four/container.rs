use crate::core::game::connect_four::instance;
use serenity::model::channel::Message;
use serenity::prelude::{RwLock, TypeMapKey};
use std::sync::Arc;
pub struct ConnectFourContainer;

impl TypeMapKey for ConnectFourContainer {
    type Value = Arc<RwLock<ConnectFourManager>>;
}

pub struct ConnectFourManager {
    pub games: Vec<instance::Instance>,
}

impl ConnectFourManager {
    pub fn new() -> ConnectFourManager {
        ConnectFourManager { games: vec![] }
    }

    pub fn add_game(&mut self, msg: Message) {
        self.games.push(instance::Instance::new(msg));
    }

    //pub fn next_move(&mut self, message_id: u64, pos: usize) {
    // if let Some(game) = self.games.iter().find(|g| g.msg.id == message_id) {}
    //}
}
