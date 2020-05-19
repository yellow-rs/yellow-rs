use crate::core::game::connect_four::board::Board;
//use crate::core::game::connect_four::board::CellState;
use serenity::model::channel::Message;
use serenity::model::user::User;

pub struct Instance {
    board: Board,
    players: Vec<User>,
    pub msg: Message,
}

impl Instance {
    pub fn new(msg: Message) -> Instance {
        Instance {
            board: Board::new(7, 6),
            players: vec![],
            msg: msg,
        }
    }

    //pub fn play(&mut self, pos: usize) {
    //  self.board.coin(pos, CellState::One);
    //}
}
