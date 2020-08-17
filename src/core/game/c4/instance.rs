use crate::core::game::c4::board_trait::C4Board;
use serenity::{http::Http, model::channel::Message};
use std::sync::Arc;

pub struct C4Instance {
    msg: Message,                 // Message to manipulate
    http: Arc<Http>,              // Http object to interact with message
    board_data: Box<dyn C4Board>, // Board data wrapper
    players_pair: [u64; 2],
    turns: u8,
    over: bool,
}

impl C4Instance {
    pub fn new(msg: Message, http: Arc<Http>, board_data: dyn C4Board) -> Self {
        C4Instance {
            msg,
            http,
            board_data,
            players_pair: [0; 2],
            turns: 1,
            over: false,
        }
    }
}
