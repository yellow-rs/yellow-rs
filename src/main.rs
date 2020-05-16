mod commands;
mod core;

use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    prelude::*,
};

use log::error;

use commands::{misc::*, play::*, tech::*, utils::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[derive(Debug)]
struct ConnectFourContainer;

impl TypeMapKey for ConnectFourContainer {
    type Value = Arc<Mutex<i32>>;
}

#[group]
#[commands(ping, avatar)]
struct Misc;

#[group]
#[commands(shards, quit)]
struct Tech;

#[group]
#[commands(add)]
struct Util;

#[group]
#[commands(c4)]
struct Play;

fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    /* Comment this line when deployed in Heroku */
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from environment*/
    env_logger::init();

    //let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(
        env::var("DISCORD_TOKEN").unwrap(),
        core::handler::ClientHandler,
    )
    .expect("Error creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
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
