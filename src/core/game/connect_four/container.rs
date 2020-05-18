use serenity::prelude::{RwLock, TypeMapKey};

use std::sync::Arc;

pub struct ConnectFourContainer;

impl TypeMapKey for ConnectFourContainer {
    type Value = Arc<RwLock<ConnectFourManager>>;
}

pub struct ConnectFourGame;

pub struct ConnectFourManager {
    pub games: Vec<ConnectFourGame>,
}

impl ConnectFourManager {
    pub fn new() -> ConnectFourManager {
        ConnectFourManager { games: vec![] }
    }
}
