mod commands;
mod telephone;

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::{HashMap, HashSet}, fs::File, io::prelude::*, sync::Arc};

use commands::{submit::*, util::*, create::*};
use telephone::{ChainStorage};
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping, submit, create)]
struct General;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    info!("Attempting to load token");
    // Configure bot with token read from file
    let mut file = File::open("oauth2.tok").expect("Error opening oauth2.tok");
    let mut token = String::new();
    file.read_to_string(&mut token)
        .expect("Error reading oauth2.tok");
    // Trim newline from token, if it exists
    token = token.trim().to_string();

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    // Brackets are to create new scope so we don't hold onto lock for too long
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ChainStorage>(HashMap::default());
    }

    // Ok so this is magic right here
    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
