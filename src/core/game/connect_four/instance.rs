use crate::core::game::connect_four::board::Board;

pub struct Instance {
    board: Board,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            board: Board::new(7, 6),
        }
    }
}
