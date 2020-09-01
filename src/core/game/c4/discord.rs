use super::{instance::C4Instance, result::GameResult};
use serenity::{
    builder::CreateEmbed,
    http::{AttachmentType, CacheHttp, Http},
    model::{channel::Message, id::ChannelId},
};
use tokio::fs::{remove_file, File};

// Called when receiving the image from the dump channel
pub fn generate_embed(gem: &C4Instance, img_link: &str) -> CreateEmbed {
    let mut result = CreateEmbed::default();

    let turn_holder = "Waiting for new players";
    let turn_subtitle = "Turn 1";

    result
        .title("ConnectFour:tm:")
        .field(turn_holder, turn_subtitle, false)
        .image(img_link)
        .url(img_link)
        .footer(|f| f.text("| Don't report bugs | Version 0.1.2 | React to place coin |"));

    result
}

// Send the reult gfx
pub async fn send_msg(file: &File, http: Http, id: u64) {
    let _ = ChannelId(617407223395647520u64)
        .send_message(http, move |m| {
            m.content(id).add_file(AttachmentType::File {
                file,
                filename: "*.png".to_string(),
            })
        })
        .await;
    remove_file(format!("{}.png", id)).await.unwrap();
}

pub async fn update_game(
    turn: usize,
    turn_holder: &str,
    img_link: &str,
    over: bool,
    players_pair: [u64; 2],
) -> Option<(u64, u64, GameResult)> {
    let mut turn_subtitle = "React to start!".to_string();
    //    let mut winner = "".to_string();
    let mut result = None;
    let mut turn_holder = turn_holder;
    if turn > 2 {
        if !over {
            //turn_holder = ;
            turn_subtitle = format!("Turn {}", turn);
        } else if turn == 43 {
            turn_holder = "Match is a draw!💣";
            turn_subtitle = "Maximum of 42 turns".to_string();

            // Delete reactions
            //let _ = self.msg.delete_reactions(&self.http).await;

            // Return result of game
            result = Some((players_pair[0], players_pair[1], GameResult::Tie));
        } else {
            //let winner_usr = &self.players_pair[(self.turns % 2) as usize];
            //turn_holder = format!("{} won! ", winner_usr.name);
            turn_subtitle = format!("completed in {} turns", turn - 1);
            //winner = winner_usr.face();
            // Delete reactions
            //let _ = self.msg.delete_reactions(&self.http).await;

            // Return result of game
            result = Some((
                1u64, //winner_usr,
                1u64, //&self.players_pair[((self.turns - 1) % 2) as usize],
                GameResult::Win,
            ));
        }
    } else {
        turn_holder = "New Player's Turn!";
    }

    result
}

async fn update_msg<H>(
    msg: &mut Message,
    http: H,
    turn_holder: &str,
    turn_subtitle: &str,
    img_link: &str,
    winner: Option<u64>,
) where
    H: CacheHttp,
{
    let _ = msg
        .edit(http, |m| {
            m.embed(|e| {
                e.title("Connect Four™")
                    .field(turn_holder, turn_subtitle, true)
                    .image(img_link)
                    .url(img_link)
                    .footer(|f| {
                        f.text("| Don't report bugs | Version 0.1.1 | React to place coin |")
                    });
                if let Some(winner) = winner {
                    add_thumbnail(e, &winner.to_string())
                }
                e
            })
        })
        .await;
}

fn add_thumbnail(embed: &mut CreateEmbed, link: &str) {
    embed.thumbnail(link);
}

// Takes a message directly and sends an embed through its channel_id
pub async fn send_embed<F>(msg: &mut Message, http: Http, embed: F)
where
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
{
    msg.channel_id
        .send_message(http, |m| m.embed(embed))
        .await
        .expect("Failed to send message");
}

// Actors
// Client => dispatch events => receive reactions
// ^                               ^
// |                               |
// ConnectFour => Processing turns and moves
//             => Generating a board
//             => Sending the board and updating the message
//             => Repeat inb4
//             => Decide a winner and update Database
//
// Solution:
//     * Use a state machine?
//     * Separate into components
//
// What does an C4 instance message looks like?
//  => Optionals:
//      * Thumbnail
//  => Changing:
//      turn_holder can also represent the winning player
//      turn subtitle can also represent the game's turn duration
//  => Constant:
//      footer - it just doesn't change
