mod commands;

use std::{collections::HashSet, env, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use log::{error, info};

use commands::{misc::*, tech::*, utils::*};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.tag());
    }
    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed!");
    }
}

#[group]
#[commands(ping, avatar)]
struct General;

#[group]
#[commands(shards, quit)]
struct Technical;

#[group]
#[commands(add)]
struct Utilities;

fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    /* Comment this line when deployed in Heroku */
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from .env*/
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").unwrap();
    //.expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    {
        client
            .data
            .write()
            .insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            Some(set)
        }
        Err(_why) => None, // panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners.unwrap()).prefix("~"))
            .group(&GENERAL_GROUP)
            .group(&TECHNICAL_GROUP)
            .group(&UTILITIES_GROUP),
    );

    if let Err(why) = client.start_autosharded() {
        error!("Client error: {:?}", why);
    }
}
