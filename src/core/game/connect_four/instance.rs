use crate::core::game::connect_four::board::{Board, C4Board};
use serenity::model::channel::Message;
use serenity::model::user::User;

pub struct Instance {
    board: Board,
    pub players: [Option<User>; 2],
    pub msg: Message,
}

impl Instance {
    pub fn new(msg: Message) -> Instance {
        Instance {
            board: Board::new(),
            players: [None, None],
            msg: msg,
        }
    }

    //pub fn play(&mut self, pos: usize) {
    //  self.board.coin(pos, CellState::One);
    //}
}
