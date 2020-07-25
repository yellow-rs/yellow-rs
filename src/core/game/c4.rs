use serenity::http::client::Http;
use serenity::model::{
    channel::Message,
    id::{MessageId, UserId},
};
use serenity::prelude::{RwLock, TypeMapKey};
use std::collections::HashMap;
use std::sync::Arc;

pub struct C4ManagerContainer;
pub type C4Manager = HashMap<MessageId, C4Instance>;

impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}

pub trait C4ManagerTrait {
    fn new_game(&mut self, http_: &Arc<Http>, msg: Message);
    fn reacted(&mut self, msg: MessageId, pos: usize, user: UserId);
}

impl C4ManagerTrait for C4Manager {
    fn new_game(&mut self, http_: &Arc<Http>, msg: Message) {
        self.insert(msg.id, C4Instance::new(msg, http_));
    }
    fn reacted(&mut self, msg_id: MessageId, pos: usize, user: UserId) {
        if pos > 0 && pos < 8 {
            if let Some(gem) = self.get_mut(&msg_id) {
                gem.move_coin(pos, user);
            }
        }
    }
}

pub struct C4Instance {
    _msg: Message,
    board: Board7By6,
    two_players: PlayersTwo,
    turns: u8,
    _http: Arc<Http>,
}

type PlayersTwo = (UserId, UserId);

impl C4Instance {
    pub fn new(_msg: Message, http_: &Arc<Http>) -> Self {
        C4Instance {
            _msg,
            board: Board7By6::new(),
            two_players: (UserId(0), UserId(0)),
            turns: 1,
            _http: Arc::clone(http_),
        }
    }

    // Checks validity of player based on turns
    pub fn move_coin(&mut self, pos: usize, user: UserId) {
        if self.turns > 2 {
            if user == self.two_players.0 || user == self.two_players.1 {
                if (self.turns % 2 == 0 && self.two_players.1 == user)
                    || (self.turns % 2 == 1 && self.two_players.0 == user)
                {
                    self.coin_drop(pos);
                }
            }
        } else if self.turns == 1 {
            self.two_players.0 = user;
            self.coin_drop(pos);
        } else if !(self.two_players.0 == user) {
            self.two_players.1 = user;
            self.coin_drop(pos);
        }
    }
    // Checks validity of move
    fn coin_drop(&mut self, pos: usize) {
        if self.board.coin(self.coin_turn(), pos - 1) {
            println!("Turn: {}", self.turns);
            self.board.dump();
            self.turns += 1;
        }
    }
    // Determine which player it should be based on turns
    fn coin_turn(&self) -> CellState {
        match self.turns % 2 == 1 {
            true => CellState::One,
            false => CellState::Two,
        }
    }
}

trait BoardPlayable {
    fn new() -> Self;
    fn coin(&mut self, new_coin: CellState, pos: usize) -> bool;
    fn dump(&self);
    fn dump_as_str(&self) -> String;
}

type Board7By6 = [[CellState; 7]; 6];

impl BoardPlayable for Board7By6 {
    fn new() -> Self {
        [[CellState::Vacant; 7]; 6]
    }
    fn coin(&mut self, new_coin: CellState, pos: usize) -> bool {
        for i in (0..6).rev() {
            if self[i][pos] == CellState::Vacant {
                self[i][pos] = new_coin;
                return true;
            }
        }
        false
    }
    fn dump(&self) {
        println!("{}", self.dump_as_str());
    }

    fn dump_as_str(&self) -> String {
        let mut result = String::new();
        for i in self.into_iter() {
            for j in i {
                result = format!("{}{:?} ", result, *j as u8);
            }
            result.push('\n');
        }
        result.push('\n');
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    One,
    Two,
}
