use log::{debug, error, info, warn};
use rmp_serde as mp;
use serenity::{
    model::channel::Message,
    model::gateway::{Activity, Ready},
    //model::id::ChannelId,
    prelude::*,
};
use std::{collections::HashMap, ops::Deref};

use crate::types;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, event: Ready) {
        debug!("got ready event: {:?}", event);
        info!(
            "logged in as {} on {} servers",
            event.user.tag(),
            event.guilds.len(),
        );
        let data = context.data.read();
        let prefix = match data.get::<types::Configuration>() {
            Some(config) => &config.prefix,
            None => panic!(
                "no configuration was stored inside of the data TypeMap (this is a severe bug)"
            ),
        };
        context.set_activity(Activity::listening(
            format!("your conversations | {}", prefix).as_str(),
        ));
    }

    fn message(&self, context: Context, message: Message) {
        debug!("got a new message: {:?}", message);

        let (channels, banned): (HashMap<u64, u64>, Vec<u64>) = {
            let data = context.data.read();
            let database = match data.get::<types::Database>() {
                Some(database) => database,
                None => panic!(
                    "the database wasn't initialized and placed into the data TypeMap (this is a severe bug)"
                ),
            };

            (
                // get the channel HashMap from the database
                database
                    .get("channels")
                    .unwrap_or_else(|e| {
                        warn!("unable to retrieve registered channels from db: {:?}", e);
                        None
                    })
                    .map_or(HashMap::new(), |v| {
                        mp::decode::from_read(v.deref()).unwrap_or_else(|e| {
                            error!("unable to unpack the channels map from the db: {:?}", e);
                            HashMap::new()
                        })
                    }),
                // get the banned users array from the database
                database
                    .get("banned_users")
                    .unwrap_or_else(|e| {
                        warn!("unable to retrieve banned users from db: {:?}", e);
                        None
                    })
                    .map_or(vec![], |v| {
                        mp::decode::from_read(v.deref()).unwrap_or_else(|e| {
                            error!(
                                "unable to unpack the banned users array from the db: {:?}",
                                e
                            );
                            vec![]
                        })
                    }),
            )
        };

        debug!("preparing to dispatch a message");
        /*
        for (_, channel) in channels {
           ChannelId(channel)
               .send_messsage(context.http, |m| m
                   .content(format!(
        */
    }
}
