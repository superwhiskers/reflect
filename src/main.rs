use log::{debug, error};
use serenity::{
    framework::standard::*, model::application::CurrentApplicationInfo, model::id::UserId,
    prelude::*,
};
use std::{collections::HashSet, fs, str::FromStr};
use toml;

mod config;
mod event_handler;
mod logger;
mod types;

fn main() {
    let config: config::Configuration = toml::from_str(
        fs::read_to_string("config.toml")
            .expect("unable to read configuration")
            .as_str(),
    )
    .expect("unable to parse configuration");

    // convert the vector into a HashSet if needed
    let mut admins = HashSet::new();
    if let Some(admin_ids) = &config.admins {
        for id in admin_ids.iter() {
            admins.insert(UserId::from_str(id).expect("unable to parse string userid to a UserId"));
        }
    }

    logger::start_logging(config.log_level).expect("unable to initiate logging");

    debug!("creating client struct");
    let mut client =
        Client::new(&config.token, event_handler::Handler).expect("unable to initiate client");

    debug!("fetching bot owner from discord");
    match client.cache_and_http.http.get_current_application_info() {
        Ok(CurrentApplicationInfo { owner, .. }) => admins.insert(owner.id),
        Err(message) => panic!("unable to get application info: {:?}", message),
    };

    client.with_framework(
        StandardFramework::new().configure(|c| c.prefix(config.prefix.as_str()).owners(admins)),
    );

    {
        debug!("storing configuration inside of the data TypeMap");
        let mut data = client.data.write();
        let _ = data.insert::<config::Configuration>(config);
    }

    // TODO(superwhiskers): implement sharding support and then switch this to be
    // "start_autosharded"
    debug!("starting bot");
    if let Err(message) = client.start() {
        error!("client exited: {:?}", message);
    }
}
