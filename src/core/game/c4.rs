use cairo::{Format, ImageSurface};
use std::f64::consts::PI;
use std::fs::File;

use serenity::{
    async_trait,
    http::{client::Http, AttachmentType},
    model::{
        channel::Message,
        id::{ChannelId, MessageId, UserId},
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
        if let Some(gem) = self.get_mut(&msg_id) {
            if !gem.blocking {
                unsafe {
                    gem.move_coin(pos, user).await;
                }
            }
        }
    }
}

pub struct C4Instance {
    msg: Message,          // Message to manipulate
    http: Arc<Http>,       // Http object to interact with message
    board_data: Board7By6, // Board data wrapper
    board_canvas: ImageSurfaceWrapper,
    players_pair: [User; 2],
    avatars: [ImageSurfaceWrapper; 2],
    turns: u8,
    blocking: bool, // Block incoming input to prevent choking operations
}

impl C4Instance {
    pub fn new(msg: Message, http: Arc<Http>) -> Self {
        C4Instance {
            msg,
            http,
            board_data: Board7By6::new(),
            board_canvas: canvas_init(),
            players_pair: [User::default(), User::default()],
            avatars: [
                ImageSurfaceWrapper::default(),
                ImageSurfaceWrapper::default(),
            ],
            turns: 1,
            blocking: false,
        }
    }

    // Checks validity of player based on turns
    pub async unsafe fn move_coin(&mut self, pos: usize, user: UserId) {
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
            // Get User
            self.players_pair[0] = self.http.get_user(user.0).await.unwrap();
            self.grab_user_avatar(0).await;
            self.coin_drop(pos).await;
        } else
        /* if !(self.two_players.0 == user)*/
        {
            self.players_pair[1] = self.http.get_user(user.0).await.unwrap();
            self.avatars[1] = self.grab_user_avatar(1).await;
            self.coin_drop(pos).await;
        }
    }
    // Checks validity of move
    async fn coin_drop(&mut self, pos: usize) {
        if self.board_data.coin(self.coin_turn(), pos - 1) {
            self.blocking = true;
            println!("Turn: {}", self.turns);

            let content = self.board_data.dump();
            self.msg
                .edit(&self.http, |m| m.content(content))
                .await
                .unwrap();
            self.turns += 1;

            let msg_send = self.update_canvas();
            self.send_msg(&msg_send).await;
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

    fn update_canvas(&mut self) -> String {
        let board = self.board_canvas.0.clone();
        let ctx = cairo::Context::new(&board);

        ctx.set_source_rgb(1.0, 1.0, 0.0);
        ctx.new_path();
        ctx.arc(29.0, 25.0, 22.0, 0.0, PI * 2.0);
        ctx.close_path();
        ctx.clip();

        let avatar = self.avatars[0].clone();

        let ctx_ava = cairo::Context::new(&avatar.0);
        ctx_ava.save();

        ctx.set_source_surface(
            &ctx_ava.get_target(),
            29.0 - avatar.0.get_width() as f64 / 2.0,
            25.0 - avatar.0.get_height() as f64 / 2.0,
        );

        let msg_id = format!("{}.png", self.msg.id.0);
        ctx.paint();
        let mut file = File::create(&msg_id).expect("Couldn't create file.");

        board
            .write_to_png(&mut file)
            .expect("Couldn’t write to png");

        self.board_canvas = ImageSurfaceWrapper(board);

        msg_id
    }

    async fn send_msg(&self, _msg_id: &String) {
        let _ = ChannelId(617407223395647520)
            .send_message(&self.http, |m| {
                m.add_file(AttachmentType::Image("assets/images/board7x6.png"))
            })
            .await;
    }

    async fn grab_user_avatar(&mut self, player: usize) -> ImageSurfaceWrapper {
        let avatar_url = self.players_pair[player]
            .face()
            .replace(".webp?size=1024", ".png?size=128");

        println!("avatar url: {}", avatar_url);
        let res = reqwest::get(&avatar_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        ImageSurfaceWrapper(ImageSurface::create_from_png(&mut res.reader()).unwrap())
    }
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Vacant,
    One,
    Two,
}

#[derive(Debug, Clone)]
struct ImageSurfaceWrapper(ImageSurface);
unsafe impl Send for ImageSurfaceWrapper {}
unsafe impl Sync for ImageSurfaceWrapper {}

fn canvas_init() -> ImageSurfaceWrapper {
    let mut board = File::open("assets/images/board7x6.png").unwrap();
    ImageSurfaceWrapper(ImageSurface::create_from_png(&mut board).unwrap())
}

impl Default for ImageSurfaceWrapper {
    fn default() -> Self {
        Self(ImageSurface::create(Format::ARgb32, 1, 1).unwrap())
    }
}
