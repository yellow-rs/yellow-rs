use serenity::model::{channel::Message, id::MessageId};
use serenity::prelude::{RwLock, TypeMapKey};
use std::collections::HashMap;
use std::sync::Arc;

pub struct C4ManagerContainer;
pub type C4Manager = HashMap<MessageId, C4Instance>;

impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}

pub trait C4ManagerTrait {
    fn new_game(&self);
}

impl C4ManagerTrait for C4Manager {
    fn new_game(&self) {
        println!("You reached the end of the line!");
    }
}

pub struct C4Instance {
    msg: Message,
    board: Board7By6,
    two_players: PlayersTwo,
    turns: u8,
}

type PlayersTwo = (u64, u64);

impl C4Instance {
    pub fn new(msg: Message) -> Self {
        C4Instance {
            msg,
            board: Board7By6::new(),
            two_players: (1, 2),
            turns: 1,
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
