use cairo::{Format, ImageSurface};

use serenity::{
    http::{client::Http, AttachmentType},
    model::{
        channel::Message,
        id::{ChannelId, MessageId, UserId},
        user::User,
    },
    prelude::{RwLock, TypeMapKey},
};
use std::{collections::HashMap, f64::consts::PI, sync::Arc};

use bytes::buf::BufExt;

pub type C4Manager = HashMap<MessageId, Arc<RwLock<C4Instance>>>;

pub struct C4ManagerContainer;
impl TypeMapKey for C4ManagerContainer {
    type Value = Arc<RwLock<C4Manager>>;
}

pub struct C4Instance {
    msg: Message,          // Message to manipulate
    http: Arc<Http>,       // Http object to interact with message
    board_data: Board7By6, // Board data wrapper
    board_canvas: ImageSurfaceWrapper,
    players_pair: [User; 2],
    avatars: [ImageSurfaceWrapper; 2],
    turns: u8,
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
        }
    }

    // Checks validity of player based on turns
    pub async unsafe fn move_coin(&mut self, pos: usize, user: UserId) {
        if self.turns > 2
            && ((self.turns % 2 == 0 && self.players_pair[1].id == user)
                || (self.turns % 2 == 1 && self.players_pair[0].id == user))
        {
            if self.turns == 42 {
                let _ = self.msg.delete_reactions(&self.http).await;
            }
            self.coin_drop(pos).await;
        } else if self.turns == 1 {
            // Get User
            self.players_pair[0] = self.http.get_user(user.0).await.unwrap();
            self.avatars[0] = self.grab_user_avatar(0).await;
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
        if let Some([col, row]) = self.board_data.coin(self.coin_turn(), pos - 1) {
            self.turns += 1;

            let msg_send = self.update_canvas([col, row]).await;
            let file = tokio::fs::File::open(&msg_send).await.unwrap();
            self.send_msg(&file).await;
        }
    }
    // Determine which player it should be based on turns
    fn coin_turn(&self) -> CellState {
        match self.turns % 2 == 1 {
            true => CellState::One,
            false => CellState::Two,
        }
    }

    async fn update_canvas(&mut self, pos: [usize; 2]) -> String {
        const ROW: [f64; 6] = [32., 95., 157., 219., 282., 345.];
        const COLUMN: [f64; 7] = [39., 104., 169., 234., 300., 365., 430.];

        let ctx = cairo::Context::new(&self.board_canvas.0);

        ctx.new_path();
        ctx.arc(COLUMN[pos[0]], ROW[pos[1]], 31.75, 0.0, PI * 2.0);
        ctx.close_path();
        ctx.clip();

        ctx.set_source_surface(
            &self.avatars[(self.turns % 2) as usize].0,
            COLUMN[pos[0]] - 32.,
            ROW[pos[1]] - 32.,
        );

        ctx.paint();

        let msg_id = format!("{}.png", self.msg.id.0);
        let mut file = std::fs::File::create(&msg_id).expect("Couldn't create file.");

        self.board_canvas
            .0
            .write_to_png(&mut file)
            .expect("Couldn’t write to png");

        msg_id
    }

    async fn send_msg(&self, file: &tokio::fs::File) {
        let _ = ChannelId(617407223395647520)
            .send_message(&self.http, move |m| {
                m.content(self.msg.id.0).add_file(AttachmentType::File {
                    file,
                    filename: "any.png".to_string(),
                })
            })
            .await;
        tokio::fs::remove_file(format!("{}.png", self.msg.id.0))
            .await
            .unwrap();
    }

    async fn grab_user_avatar(&mut self, player: usize) -> ImageSurfaceWrapper {
        let avatar_url = self.players_pair[player]
            .face()
            .replace(".webp?size=1024", ".png?size=64");

        let res = reqwest::get(&avatar_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        ImageSurfaceWrapper(ImageSurface::create_from_png(&mut res.reader()).unwrap())
    }

    pub async fn update_game(&mut self, img_link: String) {
        let turn_holder: String;
        let turn: &u8 = &self.turns;

        if self.turns > 2 {
            turn_holder = format!(
                "{}'s turn!",
                self.players_pair[(self.turns % 2) as usize].name
            );
        } else {
            turn_holder = "New Player's Turn!".to_string();
        }
        let _ = self
            .msg
            .edit(&self.http, |m| {
                m.embed(|e| {
                    e.title("Connect Four™")
                        .field(turn_holder, "React for the coin-drop", true)
                        .field("Turn", turn, true)
                        .url(&img_link)
                        .image(img_link)
                        .footer(|f| f.text("| Report bugs | Version 0.1 |"))
                })
            })
            .await;
    }
}

trait BoardPlayable {
    fn new() -> Self;
    fn coin(&mut self, coin: CellState, col: usize) -> Option<[usize; 2]>;
    fn dump(&self) -> String;
}

type Board7By6 = [[CellState; 7]; 6];

impl BoardPlayable for Board7By6 {
    fn new() -> Self {
        [[CellState::Vacant; 7]; 6]
    }
    fn coin(&mut self, coin: CellState, col: usize) -> Option<[usize; 2]> {
        for row in (0..6).rev() {
            if self[row][col] == CellState::Vacant {
                self[row][col] = coin;
                return Some([col, row]);
            }
        }
        None
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
    let mut board = std::fs::File::open("assets/images/board7x6.png").unwrap();
    ImageSurfaceWrapper(ImageSurface::create_from_png(&mut board).unwrap())
}

impl Default for ImageSurfaceWrapper {
    fn default() -> Self {
        Self(ImageSurface::create(Format::Rgb30, 128, 128).unwrap())
    }
}
