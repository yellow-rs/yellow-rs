use serenity::model::{
    channel::Message,
    id::{MessageId, UserId},
    // user::User,
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
    fn new_game(&mut self, msg: Message);
    fn reacted(&mut self, msg: MessageId, pos: usize, user: UserId);
}

impl C4ManagerTrait for C4Manager {
    fn new_game(&mut self, msg: Message) {
        self.insert(msg.id, C4Instance::new(msg));
    }
    fn reacted(&mut self, msg_id: MessageId, pos: usize, user: UserId) {
        if pos > 0 && pos < 8 {
            if let Some(gem) = self.get(&msg_id) {
                gem.move_coin(pos, user);
            }
        }
    }
}

pub struct C4Instance {
    msg: Message,
    board: Board7By6,
    two_players: PlayersTwo,
    turns: u8,
}

type PlayersTwo = (UserId, UserId);

impl C4Instance {
    pub fn new(msg: Message) -> Self {
        C4Instance {
            msg,
            board: Board7By6::new(),
            two_players: (UserId(0), UserId(0)),
            turns: 1,
        }
    }
    pub fn move_coin(&mut self, pos: usize, user: UserId) {
        if self.turns > 2 {
            if user == self.two_players.0 || user == self.two_players.1 {}
        } else if self.turns == 1 {
            self.two_players.0 = user;
        } else if !(self.two_players.0 == user) {
            self.two_players.1 = user;
        }
    }
}

trait BoardPlayable {
    fn new() -> Self;
    fn coin(&mut self, new_coin: CellState, pos: usize) -> bool;
    fn dump(&self);
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
        for i in self.into_iter() {
            for j in i {
                print!("{:?} ", *j as u8);
            }
            print!("\n");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    One,
    Two,
}
