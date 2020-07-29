use cairo::ImageSurface;
//use std::f64::consts::PI;
use std::fs::File;

use serenity::{
    async_trait,
    http::client::Http,
    model::{
        channel::Message,
        id::{MessageId, UserId},
        user::User,
    },
    prelude::{RwLock, TypeMapKey},
};

use bytes::buf::BufExt;
use std::{collections::HashMap, sync::Arc};

pub struct C4ManagerContainer;

pub type C4Manager = HashMap<MessageId, C4Instance>;

impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}

#[async_trait]
pub trait C4ManagerTrait {
    fn new_game(&mut self, http_: &Arc<Http>, msg: Message);
    async fn reacted(&mut self, msg: MessageId, pos: usize, user: UserId);
}

#[async_trait]
impl C4ManagerTrait for C4Manager {
    fn new_game(&mut self, http: &Arc<Http>, msg: Message) {
        self.insert(msg.id, C4Instance::new(msg, Arc::clone(&http)));
    }
    async fn reacted(&mut self, msg_id: MessageId, pos: usize, user: UserId) {
        if pos > 0 && pos < 8 {
            if let Some(gem) = self.get_mut(&msg_id) {
                if !gem.blocking {
                    gem.move_coin(pos, user).await;
                }
            }
        }
    }
}

pub struct C4Instance {
    msg: Message,     // Message to manipulate
    http: Arc<Http>,  // Http object to interact with message
    board: Board7By6, // Board data wrapper
    _board_canvas: ImageSurfaceWrapper,
    two_players: PlayersTwo,
    turns: u8,
    blocking: bool, // Block incoming input to prevent clashing operations
    avatars: (ImageSurfaceWrapper, ImageSurfaceWrapper),
}

type PlayersTwo = (User, User);

impl C4Instance {
    pub fn new(msg: Message, http: Arc<Http>) -> Self {
        C4Instance {
            msg,
            http,
            board: Board7By6::new(),
            _board_canvas: canvas_init(),
            two_players: (User::default(), User::default()),
            turns: 1,
            blocking: false,
            avatars: (
                ImageSurfaceWrapper::default(),
                ImageSurfaceWrapper::default(),
            ),
        }
    }

    // Checks validity of player based on turns
    pub async fn move_coin(&mut self, pos: usize, user: UserId) {
        if self.turns > 2
        /*&& ((user == self.two_players.0 || user == self.two_players.1)
        && ((self.turns % 2 == 0 && self.two_players.1 == user)
            || (self.turns % 2 == 1 && self.two_players.0 == user))) */
        {
            if self.turns == 42 {
                let _ = self.msg.delete_reactions(&self.http).await;
            }
            self.coin_drop(pos).await;
        } else if self.turns == 1 {
            self.two_players.0 = self.http.get_user(user.0).await.unwrap();
            self.coin_drop(pos).await;

            let avatar_url = self
                .two_players
                .0
                .face()
                .replace(".webp?size=1024", ".png?size=128");
            println!("avatar url: {}", avatar_url);
            let res = reqwest::get(&avatar_url)
                .await
                .expect("Failed")
                .bytes()
                .await
                .expect("");

            let img = ImageSurface::create_from_png(&mut res.reader()).unwrap();
            self.avatars.0 = ImageSurfaceWrapper {
                img_surf: Some(img),
            };
        } else
        /* if !(self.two_players.0 == user)*/
        {
            self.two_players.1 = self.http.get_user(user.0).await.unwrap();
            self.coin_drop(pos).await;

            let avatar_url = self
                .two_players
                .1
                .face()
                .replace(".webp?size=1024", ".png?size=128");
            println!("avatar url: {}", avatar_url);
            let res = reqwest::get(&avatar_url)
                .await
                .expect("Failed")
                .bytes()
                .await
                .expect("");

            let img = ImageSurface::create_from_png(&mut res.reader()).unwrap();
            self.avatars.1 = ImageSurfaceWrapper {
                img_surf: Some(img),
            };
            //let mut file = File::create(format!("{}.png", self.two_players.1.id)).unwrap();
            //img.write_to_png(&mut file).unwrap();
        }
    }
    // Checks validity of move
    async fn coin_drop(&mut self, pos: usize) {
        if self.board.coin(self.coin_turn(), pos - 1) {
            self.blocking = true;
            println!("Turn: {}", self.turns);

            let content = self.board.dump();
            self.msg
                .edit(&self.http, |m| m.content(content))
                .await
                .unwrap();
            self.turns += 1;
            self.blocking = false;
        }
    }
    // Determine which player it should be based on turns
    fn coin_turn(&self) -> CellState {
        match self.turns % 2 == 1 {
            true => CellState::One,
            false => CellState::Two,
        }
    }

    //fn update_canvas(&mut self) {}
}

trait BoardPlayable {
    fn new() -> Self;
    fn coin(&mut self, new_coin: CellState, pos: usize) -> bool;
    fn dump(&self) -> String;
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
    fn dump(&self) -> String {
        let mut result = String::new();
        for i in self.into_iter() {
            for j in i {
                result = format!("{}{:?} ", result, *j as u8);
            }
            result.push('\n');
        }
        result.push('\n');
        println!("{}", result);
        result
    }
}

#[derive(Debug)]
struct ImageSurfaceWrapper {
    img_surf: Option<ImageSurface>,
}

impl Default for ImageSurfaceWrapper {
    fn default() -> Self {
        ImageSurfaceWrapper { img_surf: None }
    }
}

unsafe impl Send for ImageSurfaceWrapper {}
unsafe impl Sync for ImageSurfaceWrapper {}

fn canvas_init() -> ImageSurfaceWrapper {
    let mut board = File::open("assets/images/board7x6.png").unwrap();
    ImageSurfaceWrapper {
        img_surf: Some(ImageSurface::create_from_png(&mut board).unwrap()),
    }
}
//fn draw() {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    One,
    Two,
}
