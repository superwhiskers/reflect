use log::error;
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use ron;
use serenity::{
    framework::standard::*, model::application::CurrentApplicationInfo, model::id::UserId,
    prelude::*,
};
use std::{collections::HashSet, fs, sync::Arc};

mod colors;
mod commands;
mod defaults;
mod event_handler;
mod logger;
mod types;
mod utils;

#[macro_use]
mod macros;

fn main() {
    let mut config: types::Configuration = ron::de::from_str(
        fs::read_to_string("config.ron")
            .expect("unable to read configuration")
            .as_str(),
    )
    .expect("unable to parse configuration");

    logger::start_logging(config.log_level, &config.log_file).expect("unable to initiate logging");

    // convert the vector into a HashSet if needed
    let mut admins = HashSet::new();
    if let Some(ids) = &config.admins {
        for id in ids.iter() {
            admins.insert(UserId(*id));
        }
    }

    // redis initialization

    let database = r2d2::Pool::builder()
        .max_size(config.database.max_connections.clone())
        .build(
            RedisConnectionManager::new(config.database.host.as_str())
                .expect("unable to create a connection manager for the redis server"),
        )
        .expect("unable to create a Pool of connections to the redis server");

    // discord initialization

    let mut client =
        Client::new(&config.token, event_handler::Handler).expect("unable to initiate client");

    match client.cache_and_http.http.get_current_application_info() {
        Ok(CurrentApplicationInfo { owner, .. }) => {
            admins.insert(owner.id);
            if let Some(admin_ids) = &mut config.admins {
                admin_ids.push(*owner.id.as_u64());
            } else {
                config.admins = Some(vec![*owner.id.as_u64()]);
            }
        }
        Err(message) => panic!("unable to get application info: {:?}", message),
    };

    // place the completed admin hashset into redis

    {
        let mut database = database
            .get()
            .expect("unable to get a connection to the redis server from the pool");
        match database
            .sadd::<&str, Vec<u64>, u64>("admins", config.admins.as_ref().unwrap().clone())
        {
            Ok(_) => (),
            Err(msg) => panic!("unable to load admin hashset into redis: {:?}", msg),
        }
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(&config.prefix.as_str().clone()).owners(admins))
            .help(&commands::help::HELP)
            .group(&commands::utility::UTILITY_GROUP)
            .group(&commands::moderation::MODERATION_GROUP),
    );

    {
        let mut data = client.data.write();

        // make some data available to event handlers & commands
        let _ = data.insert::<types::Configuration>(Arc::new(config));
        let _ = data.insert::<types::Database>(Arc::new(database));
    }

    // TODO(superwhiskers): implement sharding support and then switch this to be
    // "start_autosharded"
    if let Err(msg) = client.start() {
        error!("client exited: {:?}", msg);
    }
}
