mod commands;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::{
        event::ResumedEvent,
        gateway::Ready,
    },
    prelude::*,
};

use log::{error, info};

use commands::{
    misc::*,
};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _:Context, ready:Ready) {
        info!("Connected as {}", ready.user.tag());
    }
    fn resume(&self, _:Context, _:ResumedEvent) {
        info!("Resumed!");
    }
}

#[group]
#[commands(ping)]
struct General;

fn main() {
    /* Load env variables located at `./.env` relative to CWD*/
    kankyo::load().expect("Failed to load .env file");

    /* Initialize logger based `RUST_LOG` from .env*/
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };


    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("~"))
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
