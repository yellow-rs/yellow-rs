use crate::core::game::c4::{board_trait::C4Board, instance::C4Instance};
use serenity::prelude::{RwLock, TypeMapKey};
use std::{collections::HashMap, sync::Arc};

pub type C4Manager = HashMap<u64, C4Instance<dyn C4Board>>;

pub struct C4ManagerContainer;

impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}
