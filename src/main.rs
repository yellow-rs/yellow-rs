mod commands;
mod core;

use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

use serenity::{
    cache::{Cache, Settings},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};

use log::error;

use crate::commands::{misc::*, tech::*, utils::*};

use crate::core::{
    handler::ClientHandler,
    messagecache_container::MessageCacheContainer,
    shardmanager_container::ShardManagerContainer,
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

#[tokio::main]
async fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    /* Comment this line when deployed in Heroku */
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

    let settings = Settings::new().max_messages(1).clone();
    let _cache = Cache::new_with_settings(settings);

    let mut client = Client::new(&token)
        .framework(
            StandardFramework::new()
                .configure(|c| c.owners(owners).prefix("/"))
                .help(&HELP)
                .group(&MISC_GROUP)
                .group(&TECH_GROUP)
                .group(&UTIL_GROUP),
        )
        .event_handler(ClientHandler)
        .await
        .expect("");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<MessageCacheContainer>(Arc::new(RwLock::new(HashMap::new())));
    }

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
