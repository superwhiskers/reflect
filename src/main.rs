use log::error;
use cdrs;
use serenity::{
    framework::standard::*, model::application::CurrentApplicationInfo, model::id::UserId,
    prelude::*,
};
use std::{collections::HashSet, fs, str::FromStr, sync::Arc};
use ron;

mod defaults;
mod event_handler;
mod logger;
mod types;
mod commands;

fn main() {
    let config: types::Configuration = ron::de::from_str(
        fs::read_to_string("config.ron")
            .expect("unable to read configuration")
            .as_str(),
    )
    .expect("unable to parse configuration");

    logger::start_logging(config.log_level, &config.log_file).expect("unable to initiate logging");

    // convert the vector into a HashSet if needed
    let mut admins = HashSet::new();
    if let Some(admin_ids) = &config.admins {
        for id in admin_ids.iter() {
            admins.insert(UserId::from_str(id).expect("unable to parse string userid to a UserId"));
        }
    }

    // cassandra initialization

    let mut database = {
        let mut hosts = Vec::new();
        for host in &config.database.hosts {
            hosts.push(cdrs::cluster::NodeTcpConfigBuilder::new(&host.host, &host.username.map_or_else(cdrs::authenticators::NoneAuthenticator{},|u| cdrs::authenticators::StaticPasswordAuthenticator::new(u, &host.password.unwrap_or("")))).build());
        }
        let mut database = cdrs::cluster::session::new(cdrs::cluster::ClusterTcpConfig(hosts), cdrs::load_balancing::RoundRobin::new()).expect("unable to create database session");
        database.compression = &config.database.compression;
        database
    };

    // discord initialization

    let mut client =
        Client::new(&config.token, event_handler::Handler).expect("unable to initiate client");

    match client.cache_and_http.http.get_current_application_info() {
        Ok(CurrentApplicationInfo { owner, .. }) => admins.insert(owner.id),
        Err(message) => panic!("unable to get application info: {:?}", message),
    };

    client.with_framework(
        StandardFramework::new().configure(|c| c.prefix(config.prefix.as_str()).owners(admins)).group(&commands::utility::UTILITY_GROUP),
    );

    {
        let mut data = client.data.write();

        // make some data available to event handlers & commands
        let _ = data.insert::<types::Configuration>(Arc::new(config));
        let _ = data.insert::<types::Database>(Arc::new(database));
    }

    // TODO(superwhiskers): implement sharding support and then switch this to be
    // "start_autosharded"
    if let Err(message) = client.start() {
        error!("client exited: {:?}", message);
    }
}
