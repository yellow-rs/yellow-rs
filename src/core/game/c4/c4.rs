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
            players_pair: [0; 2],
            turns: 1,
            over: false,
        }
    }

    pub async fn move_coin(&mut self, pos: usize, user: UserId) {
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

pub async fn send_embed<F>(&mut msg: Message, embed: F)
where
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
{
    msg.channel_id
        .send_message(&self.http, |m| m.embed(embed))
        .await
        .expect("Failed to send message");
}
