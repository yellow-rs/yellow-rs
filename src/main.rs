mod commands;
mod core;

use serenity::{
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use log::error;

use crate::commands::{misc::*, play::*, tech::*, utils::*};

use crate::core::{
    game::c4::{C4Manager, C4ManagerContainer},
    handler::ClientHandler,
    shardmanager_container::ShardManagerContainer,
    db::DatabaseWrapper,
};

#[group]
#[commands(avatar, sudo)]
struct Misc;

#[group]
#[commands(quit, shards, ping)]
struct Tech;

#[group]
#[commands(eval)]
struct Util;

#[group]
#[commands(connect_four, leaderboard)]
struct Play;

#[tokio::main]
async fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from environment*/
    pretty_env_logger::init_timed();

    let token = env::var("DISCORD_TOKEN").unwrap();

    let http = Http::new_with_token(&token);

    let (owners, _) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    let mut client = Client::new(&token)
        .framework(
            StandardFramework::new()
                .configure(|c| {
                    c.owners(owners)
                        .prefix("/")
                        .no_dm_prefix(true)
                        .case_insensitivity(true)
                })
                .help(&HELP)
                .group(&MISC_GROUP)
                .group(&TECH_GROUP)
                .group(&UTIL_GROUP)
                .group(&PLAY_GROUP),
        )
        .event_handler(ClientHandler)
        .await
        .expect("");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<C4ManagerContainer>(Arc::new(RwLock::new(C4Manager::new())));
        client.cache_and_http.cache.set_max_messages(20).await;
        
        // DB stuff
        // Connect to the database
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres", tokio_postgres::NoTls).await.expect("Failed to connect to postgresql database");

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let db_wrapper = DatabaseWrapper::new(client);
        match db_wrapper.generate_tables().await {
            Ok(_) => {}
            Err(why) => {
                panic!("Failed to generate postgres tables: {}", why);
            }
        };

        // db_wrapper.update_score(1, 2, crate::core::game::GameResult::Loose).await;

        data.insert::<DatabaseWrapper>(db_wrapper);
    }

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
