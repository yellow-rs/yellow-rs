mod commands;
mod core;

use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    framework::{standard::macros::group, StandardFramework},
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
#[commands(ping, avatar, verified)]
struct Misc;

#[group]
#[commands(shards, quit)]
struct Tech;

#[group]
#[commands(add)]
struct Util;

#[group]
#[commands(c4, games)]
struct Play;

fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    /* Comment this line when deployed in Heroku */
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from environment*/
    env_logger::init();

    let mut client = Client::new(env::var("DISCORD_TOKEN").unwrap(), ClientHandler)
        .expect("Error creating client");

    //let c4 = ConnectFourManager::new();
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ConnectFourContainer>(Arc::new(RwLock::new(ConnectFourManager::new())));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            Some(set)
        }
        Err(_why) => None,
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners.unwrap()).prefix("~"))
            .group(&MISC_GROUP)
            .group(&TECH_GROUP)
            .group(&UTIL_GROUP)
            .group(&PLAY_GROUP),
    );

    if let Err(why) = client.start_autosharded() {
        error!("Client error: {:?}", why);
    }
}
