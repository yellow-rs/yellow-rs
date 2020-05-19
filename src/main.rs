mod commands;
mod core;

use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};

use log::error;

use crate::commands::{misc::*, play::*, tech::*, utils::*};

use crate::core::{
    game::connect_four::container::{ConnectFourContainer, ConnectFourManager},
    handler::ClientHandler,
    shardmanager_container::ShardManagerContainer,
};

#[group]
#[commands(avatar)]
struct Misc;

#[group]
#[commands(quit, shards)]
struct Tech;

#[group]
#[commands(add)]
struct Util;

#[group]
#[commands(connect_four, games)]
struct Play;

#[tokio::main]
async fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    /* Comment this line when deployed in Heroku */
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from environment*/
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").unwrap();

    let http = Http::new_with_token(&token);

    let (owners, id) = match http.get_current_application_info().await {
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
                .configure(|c| c.owners(owners).prefix("~"))
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
        data.insert::<ConnectFourContainer>(Arc::new(RwLock::new(ConnectFourManager::new())));
    }

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
