use cairo::{Format, ImageSurface};

use crate::core::game::GameResult;
use serenity::{
    builder::CreateEmbed,
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
            over: false,
        }
    }

    pub async fn send_embed<F>(&mut self, embed: F)
    where
        F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
    {
        self.msg
            .channel_id
            .send_message(&self.http, |m| m.embed(embed))
            .await
            .expect("Failed to send message");
    }

    // Checks validity of player based on turns
    pub async unsafe fn move_coin(&mut self, pos: usize, user: UserId) {
        if !self.over
            && self.turns > 2
            && ((self.turns % 2 == 0 && self.players_pair[1].id == user)
                || (self.turns % 2 == 1 && self.players_pair[0].id == user))
        {
            self.coin_drop(pos).await;
        } else if self.turns == 1 {
            // Get User
            self.players_pair[0] = self.http.get_user(user.0).await.unwrap();
            self.avatars[0] = self.grab_user_avatar(0).await;
            self.coin_drop(pos).await;
        } else if self.turns == 2 && !(self.players_pair[0].id == user) {
            self.players_pair[1] = self.http.get_user(user.0).await.unwrap();
            self.avatars[1] = self.grab_user_avatar(1).await;
            self.coin_drop(pos).await;
        }
    }
    // Checks validity of move
    async fn coin_drop(&mut self, pos: usize) {
        if let Ok([col, row]) = self.board_data.coin(self.coin_turn(), pos - 1) {
            self.turns += 1;

            let msg_send = self.update_canvas([col, row]).await;
            let file = tokio::fs::File::open(&msg_send).await.unwrap();
            self.send_msg(&file).await;
            self.over = self.board_data.check(self.coin_turn().flip(), [row, col]);
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
        let face = &self.players_pair[player].face();
        let avatar_url = format!("{}.png?size=64", face.rsplitn(2, ".").nth(1).unwrap());

        let res = reqwest::get(&avatar_url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        ImageSurfaceWrapper(ImageSurface::create_from_png(&mut res.reader()).unwrap())
    }

    pub async fn update_game(&mut self, img_link: &str) -> Option<(&User, &User, GameResult)> {
        let turn_holder: String;
        let turn = self.turns;
        let mut turn_subtitle = "React to start!".to_string();
        let mut winner = "".to_string();
        let mut result = None;

        if self.turns > 2 {
            if !self.over {
                turn_holder = format!(
                    "{}'s turn!",
                    self.players_pair[((self.turns - 1) % 2) as usize].name
                );
                turn_subtitle = format!("Turn {}", turn);
            } else if self.turns == 43 {
                turn_holder = "Match is a draw!💣".to_string();
                turn_subtitle = "Maximum of 42 turns".to_string();

                // Delete reactions
                let _ = self.msg.delete_reactions(&self.http).await;

                // Return result of game
                result = Some((
                    &self.players_pair[0],
                    &self.players_pair[1],
                    GameResult::Tie,
                ));
            } else {
                let winner_usr = &self.players_pair[(self.turns % 2) as usize];
                turn_holder = format!("{} won! ", winner_usr.name);
                turn_subtitle = format!("completed in {} turns", turn - 1);
                winner = winner_usr.face();

                // Delete reactions
                let _ = self.msg.delete_reactions(&self.http).await;

                // Return result of game
                result = Some((
                    winner_usr,
                    &self.players_pair[((self.turns - 1) % 2) as usize],
                    GameResult::Win,
                ));
            }
        } else {
            turn_holder = "New Player's Turn!".to_string();
        }

        let _ = self
            .msg
            .edit(&self.http, |m| {
                m.embed(|e| {
                    e.title("Connect Four™")
                        .field(turn_holder, turn_subtitle, true)
                        .image(img_link)
                        .url(img_link)
                        .footer(|f| {
                            f.text("| Don't report bugs | Version 0.1.1 | React to place coin |")
                        });
                    if !winner.is_empty() {
                        add_thumbnail(e, &winner)
                    }
                    e
                })
            })
            .await;

        result
    }
}

fn add_thumbnail(embed: &mut CreateEmbed, link: &str) {
    embed.thumbnail(link);
}

type Board7By6 = [[CellState; 7]; 6];

impl BoardPlayable for Board7By6 {
    fn new() -> Self {
        [[CellState::Vacant; 7]; 6]
    }
    fn coin(&mut self, coin: CellState, col: usize) -> Result<[usize; 2], ()> {
        for row in (0..6).rev() {
            if self[row][col] == CellState::Vacant {
                self[row][col] = coin;
                return Ok([col, row]);
            }
        }
        Err(())
    }
    fn check(&self, coin: CellState, pos: [usize; 2]) -> bool {
        let mut acc = 0u8;
        // Vertical check
        for row in self.iter() {
            if row[pos[1]] == coin {
                acc += 1;
                if acc == 4 {
                    return true;
                }
            } else {
                acc = 0;
            }
        }

        // Horizontal check
        acc = 0;
        for cell in self[pos[0]].iter() {
            if cell == &coin {
                acc += 1;
                if acc == 4 {
                    return true;
                }
            } else {
                acc = 0
            }
        }

        acc = 1;
        let coefficient = pos[0] as i8 - pos[1] as i8;
        if coefficient < 3 && coefficient > -4 {
            let mut j = pos[1];

            for i in (0..pos[0]).rev() {
                if j != 0 {
                    j -= 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            for i in (pos[0] + 1)..6 {
                if j != 6 {
                    j += 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if acc >= 4 {
                return true;
            }
        }

        acc = 1;
        let coefficient = pos[0] + pos[1];
        if coefficient > 2 && coefficient < 9 {
            let mut j = pos[1];
            for i in (0..pos[0]).rev() {
                if j != 6 {
                    j += 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            j = pos[1];
            for i in (pos[0] + 1)..6 {
                if j != 0 {
                    j -= 1;
                    if self[i][j] == coin {
                        acc += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if acc >= 4 {
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
        result
    }
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
