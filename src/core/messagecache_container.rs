use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::prelude::{RwLock, TypeMapKey};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MessageCacheContainer;

impl TypeMapKey for MessageCacheContainer {
    type Value = Arc<RwLock<HashMap<ChannelId, Message>>>;
}
